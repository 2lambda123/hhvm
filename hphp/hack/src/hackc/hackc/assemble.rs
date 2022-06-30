// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.

use crate::regex;
use crate::FileOpts;
use anyhow::{anyhow, bail, Result};
use bstr::ByteSlice;
use bumpalo::Bump;
use clap::Parser;
use ffi::{Maybe, Pair, Slice, Str};
use options::Options;
use oxidized::relative_path::{self, RelativePath};
use rayon::prelude::*;
use regex::bytes::Regex;
use std::{
    collections::{HashMap, VecDeque},
    ffi::OsString,
    fmt,
    fs::{self, File},
    io::{stdout, Write},
    os::unix::ffi::OsStringExt,
    path::{Path, PathBuf},
    str::FromStr,
    sync::Mutex,
};

pub fn run(mut opts: Opts) -> Result<()> {
    let writer: SyncWrite = match &opts.output_file {
        None => Mutex::new(Box::new(stdout())),
        Some(output_file) => Mutex::new(Box::new(File::create(output_file)?)),
    };
    let files = opts.files.gather_input_files()?;
    files
        .into_par_iter()
        .map(|path| process_one_file(&path, &opts, &writer))
        .collect::<Vec<_>>()
        .into_iter()
        .collect()
}

/// Assemble the hhas in a given file to a HackCUnit. Then use bytecode printer
/// to write the hhas representation of that HCU to output
pub fn process_one_file(f: &Path, opts: &Opts, w: &SyncWrite) -> Result<()> {
    let alloc = Bump::default();
    let (hcu, fp) = assemble(&alloc, f, opts)?; // Assemble will print the tokens to output
    let filepath = RelativePath::make(relative_path::Prefix::Dummy, fp);
    let comp_options: Options = Default::default();
    let ctxt = bytecode_printer::Context::new(&comp_options, Some(&filepath), false);
    let mut output = Vec::new();
    match bytecode_printer::print_unit(&ctxt, &mut output, &hcu) {
        Err(e) => {
            eprintln!("Error bytecode_printing file {}: {}", f.display(), e);
            Err(anyhow!("bytecode_printer problem"))
        }
        Ok(_) => {
            w.lock().unwrap().write_all(&output)?;
            Ok(())
        }
    }
}

/// Assembles the hhas within f to a HackCUnit
pub fn assemble<'arena>(
    alloc: &'arena Bump,
    f: &Path,
    _opts: &Opts,
) -> Result<(hhbc::hackc_unit::HackCUnit<'arena>, PathBuf)> {
    let s: Vec<u8> = fs::read(f)?;
    assemble_from_bytes(alloc, &s)
}

/// Assembles the hhas represented by the slice of bytes input
pub fn assemble_from_bytes<'arena>(
    alloc: &'arena Bump,
    s: &[u8],
) -> Result<(hhbc::hackc_unit::HackCUnit<'arena>, PathBuf)> {
    let mut lex = Lexer::from_slice(s, 1);
    assemble_from_toks(alloc, &mut lex)
}

/// Assembles the HCU. Parses over the top level of the .hhas file
/// File is either empty OR looks like:
/// .filepath <str_literal>
/// (.function <...>)+
/// (.function_refs <...>)+
fn assemble_from_toks<'arena, 'a>(
    alloc: &'arena Bump,
    token_iter: &mut Lexer<'a>,
) -> Result<(hhbc::hackc_unit::HackCUnit<'arena>, PathBuf)> {
    let mut funcs = Vec::new();
    let mut adatas = Vec::new();
    let mut func_refs = None; // Only one func_refs which is itself a list, but that's inside the func_refs object
    let mut fp = None; // Option<PathBuf>
    let mut fatal = None; // Maybe<ffi::Triple<hhbc::FatalOp, hhbc::hhas_pos::HhasPos, Str<'arena>>>
    while token_iter.peek().is_some() {
        if token_iter.peek_if_str(Token::is_decl, ".filepath") {
            if fp.is_some() {
                bail!(".filepath specified more than once in file");
            }
            fp = Some(assemble_filepath(token_iter)?);
        } else if token_iter.peek_if_str(Token::is_decl, ".fatal") {
            if fp.is_none() {
                bail!("No .filepath specified in file. Must be specified before other headers");
            }
            fatal = Some(assemble_fatal(alloc, token_iter)?);
        } else if token_iter.peek_if_str(Token::is_decl, ".adata") {
            if fp.is_none() {
                bail!("No .filepath specified in file. Must be specified before other headers");
            }
            adatas.push(assemble_adata(alloc, token_iter)?);
        } else if token_iter.peek_if_str(Token::is_decl, ".function") {
            if fp.is_none() {
                bail!("No .filepath specified in file. Must be specified before other headers");
            }
            funcs.push(assemble_function(alloc, token_iter)?);
        } else if token_iter.peek_if_str(Token::is_decl, ".function_refs") {
            if fp.is_none() {
                bail!("No .filepath specified in file. Must be specified before other headers");
            }
            if func_refs.is_some() {
                bail!("Func refs defined multiple times in file");
            }
            func_refs = Some(assemble_function_refs(alloc, token_iter)?);
        } else {
            bail!(
                "Unknown top level identifier: {}",
                token_iter.next().unwrap()
            )
        }
    }
    let hcu = hhbc::hackc_unit::HackCUnit {
        adata: Slice::fill_iter(alloc, adatas.into_iter()),
        functions: Slice::fill_iter(alloc, funcs.into_iter()),
        classes: Default::default(),
        typedefs: Default::default(),
        file_attributes: Default::default(),
        modules: Default::default(),
        module_use: Maybe::Nothing,
        symbol_refs: hhbc::hhas_symbol_refs::HhasSymbolRefs {
            functions: func_refs.unwrap_or_default(),
            ..Default::default()
        },
        constants: Default::default(),
        fatal: fatal.into(),
    };
    if let Some(fp) = fp {
        Ok((hcu, fp))
    } else {
        bail!("No .filepath specified in file")
    }
}

/// Ex: .fatal 2:63,2:63 Parse "A right parenthesis `)` is expected here.";
fn assemble_fatal<'arena, 'a>(
    alloc: &'arena Bump,
    token_iter: &mut Lexer<'a>,
) -> Result<ffi::Triple<hhbc::FatalOp, hhbc::hhas_pos::HhasPos, Str<'arena>>> {
    token_iter.expect_is_str(Token::into_decl, ".fatal")?;
    let pos = hhbc::hhas_pos::HhasPos {
        line_begin: token_iter.expect_and_get_number()?,
        col_begin: {
            token_iter.expect(Token::into_colon)?;
            token_iter.expect_and_get_number()?
        },
        line_end: {
            token_iter.expect(Token::into_comma)?;
            token_iter.expect_and_get_number()?
        },
        col_end: {
            token_iter.expect(Token::into_colon)?;
            token_iter.expect_and_get_number()?
        },
    };
    let fat_op = token_iter.expect(Token::into_identifier)?;
    let fat_op = match fat_op {
        b"Parse" => hhbc::FatalOp::Parse,
        b"Runtime" => hhbc::FatalOp::Runtime,
        b"RuntimeOmitFrame" => hhbc::FatalOp::RuntimeOmitFrame,
        _ => bail!("Unknown fatal op: {:?}", fat_op),
    };
    let msg = escaper::unquote_slice(token_iter.expect(Token::into_str_literal)?);
    token_iter.expect(Token::into_semicolon)?;
    Ok(ffi::Triple::from((fat_op, pos, Str::new_slice(alloc, msg))))
}

/// A line of adata looks like:
/// .adata id = """<tv>"""
/// with tv being a typed value; see `assemble_typed_value` doc for what <tv> looks like.
fn assemble_adata<'arena, 'a>(
    alloc: &'arena Bump,
    token_iter: &mut Lexer<'a>,
) -> Result<hhbc::hhas_adata::HhasAdata<'arena>> {
    token_iter.expect_is_str(Token::into_decl, ".adata")?;
    let id = Str::new_slice(alloc, token_iter.expect(Token::into_identifier)?);
    token_iter.expect(Token::into_equal)?;
    // What's left here is tv
    let (tv, tv_line) = token_iter.expect(Token::into_triple_str_literal_and_line)?;
    debug_assert!(&tv[0..3] == b"\"\"\"" && &tv[tv.len() - 3..tv.len()] == b"\"\"\"");
    let tv = &tv[3..tv.len() - 3];
    let tv = &escaper::unescape_literal_bytes_into_vec_bytes(tv)?;
    // Have to unescape tv -- for example, """D:1:{s:5:\"class\"; D:...""" causes error parsing \"class\"
    let mut tv_lexer = Lexer::from_slice(tv, tv_line);
    // this so it's accurate again?
    let value = assemble_typed_value(alloc, &mut tv_lexer)?;
    tv_lexer.expect_end()?;
    token_iter.expect(Token::into_semicolon)?;
    Ok(hhbc::hhas_adata::HhasAdata { id, value })
}

/// tv can look like:
/// uninit | N; | s:s.len():"(escaped s)"; | l:s.len():"(escaped s)"; | d:#; | i:#; | b:0; | b:1; | D:dict.len():{((tv1);(tv2);)*}
/// | v:vec.len():{(tv;)*} | k:keyset.len():{(tv;)*}
fn assemble_typed_value<'arena, 'a>(
    // can use this for arguments of user_attrs as well .v.
    alloc: &'arena Bump,
    token_iter: &mut Lexer<'a>,
) -> Result<hhbc::TypedValue<'arena>> {
    let tok = token_iter
        .peek()
        .ok_or_else(|| anyhow!("No typed value to parse"))?;
    match tok.as_bytes() {
        b"uninit" => {
            token_iter.next();
            Ok(hhbc::TypedValue::Uninit)
        }
        b"N" => {
            token_iter.expect_is_str(Token::into_identifier, "N")?;
            token_iter.expect(Token::into_semicolon)?;
            Ok(hhbc::TypedValue::Null)
        }
        b"s" => {
            // String
            token_iter.expect_is_str(Token::into_identifier, "s")?;
            token_iter.expect(Token::into_colon)?;
            token_iter.expect(Token::into_number)?;
            token_iter.expect(Token::into_colon)?;
            let st = token_iter.expect(Token::into_str_literal)?;
            token_iter.expect(Token::into_semicolon)?;
            Ok(hhbc::TypedValue::string(Str::new_slice(alloc, st))) // Have to check escaping
        }
        b"l" => todo!(), // LazyClass
        b"d" => todo!(), // Float
        b"i" => {
            // Int
            token_iter.expect_is_str(Token::into_identifier, "i")?;
            token_iter.expect(Token::into_colon)?;
            let num: i64 = token_iter.expect_and_get_number()?;
            token_iter.expect(Token::into_semicolon)?;
            Ok(hhbc::TypedValue::Int(num))
        }
        b"b" => {
            // Bool
            token_iter.expect_is_str(Token::into_identifier, "b")?;
            token_iter.expect(Token::into_colon)?;
            let num = token_iter.expect_and_get_number()?;
            token_iter.expect(Token::into_semicolon)?;
            match num {
                0 => Ok(hhbc::TypedValue::Bool(false)),
                1 => Ok(hhbc::TypedValue::Bool(true)),
                _ => bail!("Given bool adata specified as number other than 0 or 1"),
            }
        }
        b"v" => assemble_tv_vec(alloc, token_iter), // Vec
        b"k" => assemble_tv_keyset(alloc, token_iter), // Keyset
        b"D" => assemble_tv_dict(alloc, token_iter), // Dict
        _ => bail!("Unknown typed value passed to .adata: {}", tok),
    }
}

// For these collection data types, `print_pos_as_prov_tag` is called in bytecode_printer if
// context has `array_provenance` true... but rarely happens and not sure how to parse that, so todo!()
fn assemble_tv_vec<'arena, 'a>(
    alloc: &'arena Bump,
    token_iter: &mut Lexer<'a>,
) -> Result<hhbc::TypedValue<'arena>> {
    token_iter.expect_is_str(Token::into_identifier, "v")?;
    token_iter.expect(Token::into_colon)?;
    let ln: usize = token_iter.expect_and_get_number()?;
    token_iter.expect(Token::into_colon)?;
    token_iter.expect(Token::into_open_curly)?;
    let mut vec = Vec::with_capacity(ln);
    for _ in 0..ln {
        vec.push(assemble_typed_value(alloc, token_iter)?);
    }
    token_iter.expect(Token::into_close_curly)?;
    Ok(hhbc::TypedValue::Vec(Slice::from_vec(alloc, vec)))
}

/// D:(D.len):{p1_0; p1_1; ...; pD.len_0; pD.len_1}
fn assemble_tv_dict<'arena, 'a>(
    alloc: &'arena Bump,
    token_iter: &mut Lexer<'a>,
) -> Result<hhbc::TypedValue<'arena>> {
    token_iter.expect_is_str(Token::into_identifier, "D")?;
    token_iter.expect(Token::into_colon)?;
    let ln: usize = token_iter.expect_and_get_number()?; // Length is the # of pairs in the dict
    token_iter.expect(Token::into_colon)?;
    token_iter.expect(Token::into_open_curly)?;
    let mut dict: Vec<Pair<hhbc::TypedValue<'arena>, hhbc::TypedValue<'arena>>> =
        Vec::with_capacity(ln);
    for _ in 0..ln {
        dict.push(Pair::from((
            assemble_typed_value(alloc, token_iter)?,
            assemble_typed_value(alloc, token_iter)?,
        )));
    }
    token_iter.expect(Token::into_close_curly)?;
    Ok(hhbc::TypedValue::Dict(Slice::from_vec(alloc, dict)))
}

fn assemble_tv_keyset<'arena, 'a>(
    _alloc: &'arena Bump,
    _token_iter: &mut Lexer<'a>,
) -> Result<hhbc::TypedValue<'arena>> {
    todo!()
}

/// Function ref looks like:
/// .functionrefs {
///    (identifier)+
/// }
///
fn assemble_function_refs<'arena, 'a>(
    alloc: &'arena Bump,
    token_iter: &mut Lexer<'a>,
) -> Result<Slice<'arena, hhbc::FunctionName<'arena>>> {
    token_iter.expect_is_str(Token::into_decl, ".function_refs")?;
    token_iter.expect(Token::into_open_curly)?;
    let mut fn_names = Vec::new();
    while !token_iter.peek_if(Token::is_close_curly) {
        let nm = assemble_name(alloc, token_iter)?; // This fails if sees non-identifier. So enforces the grammar rule of this component
        fn_names.push(nm);
    }
    token_iter.expect(Token::into_close_curly)?;
    Ok(Slice::from_vec(alloc, fn_names))
}

/// A function def is composed of the following:
/// .function {upper bounds} [special_and_user_attrs] (span) <type_info> name (params) flags? {body}
fn assemble_function<'arena, 'a>(
    alloc: &'arena Bump,
    token_iter: &mut Lexer<'a>,
) -> Result<hhbc::hhas_function::HhasFunction<'arena>> {
    token_iter.expect_is_str(Token::into_decl, ".function")?;
    let upper_bounds = assemble_upper_bounds(alloc, token_iter)?;
    // Special and user attrs may or may not be specified. If not specified, no [] printed
    let (attr, attributes) = assemble_special_and_user_attrs(alloc, token_iter)?;
    let span = assemble_span(token_iter)?;
    // Body may not have return type info, so check if next token is a < or not
    // Specifically if body doesn't have a return type info bytecode printer doesn't print anything
    // (doesn't print <>)
    let return_type_info = match token_iter.peek() {
        Some(Token::Lt(_)) => assemble_type_info(alloc, token_iter)?,
        _ => Maybe::Nothing,
    };
    // Assemble_name
    let name = assemble_name(alloc, token_iter)?;
    let mut decl_map: HashMap<&[u8], u32> = HashMap::new(); // Will store decls in this order: params, decl_vars, unnamed
    let params = assemble_params(alloc, token_iter, &mut decl_map)?;
    let flags = assemble_function_flags(token_iter)?;
    let partial_body = assemble_body(alloc, token_iter, &mut decl_map)?; // Takes ownership of decl_map
    // Fill partial_body in with params, return_type_info, and bd_upper_bounds
    let body = hhbc::hhas_body::HhasBody {
        params,
        return_type_info,
        upper_bounds,
        ..partial_body
    };
    let hhas_func = hhbc::hhas_function::HhasFunction {
        attributes,
        name,
        body,
        span,
        // Not too sure where the bottom stuff come from
        coeffects: Default::default(), // Get this from the body -- it's printed there (?)
        flags,
        attrs: attr,
    };
    Ok(hhas_func)
}

/// Have to parse flags which may or may not appear: isGenerator isAsync isPairGenerator
fn assemble_function_flags<'a>(
    token_iter: &mut Lexer<'a>,
) -> Result<hhbc::hhas_function::HhasFunctionFlags> {
    let mut flag = hhbc::hhas_function::HhasFunctionFlags::empty();
    while token_iter.peek_if(Token::is_identifier) {
        match token_iter.expect(Token::into_identifier)? {
            b"isPairGenerator" => flag |= hhbc::hhas_function::HhasFunctionFlags::PAIR_GENERATOR,
            b"isAsync" => flag |= hhbc::hhas_function::HhasFunctionFlags::ASYNC,
            b"isGenerator" => flag |= hhbc::hhas_function::HhasFunctionFlags::GENERATOR,
            f => bail!("Unknown function flag: {:?}", f),
        }
    }
    Ok(flag)
}

/// Parses over filepath. Note that HCU doesn't hold the filepath; filepath is in the context passed to the
/// bytecode printer. So we don't store the filepath in our HCU or anywhere, but still parse over it
/// Filepath: .filepath strliteral semicolon (.filepath already consumed in `assemble_from_toks)
fn assemble_filepath<'a>(token_iter: &mut Lexer<'a>) -> Result<PathBuf> {
    // Filepath is .filepath strliteral and semicolon
    token_iter.expect_is_str(Token::into_decl, ".filepath")?;
    let fp = token_iter.expect(Token::into_str_literal)?;
    let fp = escaper::unquote_slice(fp);
    let fp = PathBuf::from(OsString::from_vec(fp.to_vec()));
    token_iter.expect(Token::into_semicolon)?;
    Ok(fp)
}

/// Span ex: (2, 4)
fn assemble_span<'a>(token_iter: &mut Lexer<'a>) -> Result<hhbc::hhas_pos::HhasSpan> {
    token_iter.expect(Token::into_open_paren)?;
    let line_begin: usize = token_iter.expect_and_get_number()?;
    token_iter.expect(Token::into_comma)?;
    let line_end: usize = token_iter.expect_and_get_number()?;
    token_iter.expect(Token::into_close_paren)?;
    Ok(hhbc::hhas_pos::HhasSpan {
        line_begin,
        line_end,
    })
}

/// Ex: {(T as <"HH\\int" "HH\\int" upper_bound>)}
fn assemble_upper_bounds<'arena, 'a>(
    _alloc: &'arena Bump,
    token_iter: &mut Lexer<'a>,
) -> Result<Slice<'arena, Pair<Str<'arena>, Slice<'arena, hhbc::hhas_type::HhasTypeInfo<'arena>>>>>
{
    token_iter.expect(Token::into_open_curly)?;
    // Pass over everything until the next close curly
    while !token_iter.peek_if(Token::is_close_curly) {
        todo!(); // Don't know how to parse the inside of upper bounds yet
    }
    token_iter.expect(Token::into_close_curly)?;
    Ok(Default::default())
}

/// Currently expects everything within the braces to be user_attrs (as in, not hhvm_types_ffi::Attr)
/// Ex: [ "__EntryPoint"("""v:0:{}""")]. This example lacks Attrs
fn assemble_special_and_user_attrs<'arena, 'a>(
    alloc: &'arena Bump,
    token_iter: &mut Lexer<'a>,
) -> Result<(
    hhvm_types_ffi::ffi::Attr,
    Slice<'arena, hhbc::hhas_attribute::HhasAttribute<'arena>>,
)> {
    let mut user_atts = Vec::new();
    if token_iter.next_if(Token::is_open_bracket) {
        while !token_iter.peek_if(Token::is_close_bracket) {
            // If the token isn't a string literal then it's an Attr (not a user_attr/HhasAttribute), so todo!() that
            if token_iter.peek_if(Token::is_str_literal) {
                user_atts.push(assemble_user_attr(alloc, token_iter)?)
            } else {
                todo!()
            }
        }
        token_iter.expect(Token::into_close_bracket)?;
    }
    // If no special and user attrs then no [] printed
    let user_atts = Slice::from_vec(alloc, user_atts);
    Ok((hhvm_types_ffi::ffi::Attr::AttrNone, user_atts))
}

/// HhasAttributes are printed as follows:
/// "name"("""v:args.len:{args}""") where args are typed values.
fn assemble_user_attr<'arena, 'a>(
    alloc: &'arena Bump,
    token_iter: &mut Lexer<'a>,
) -> Result<hhbc::hhas_attribute::HhasAttribute<'arena>> {
    let nm = escaper::unescape_bytes(escaper::unquote_slice(
        token_iter.expect(Token::into_str_literal)?,
    ))?;
    let name = Str::new_slice(alloc, &nm);
    token_iter.expect(Token::into_open_paren)?;
    let (args, args_line) = token_iter.expect(Token::into_triple_str_literal_and_line)?;
    debug_assert!(&args[0..3] == b"\"\"\"" && &args[args.len() - 3..args.len()] == b"\"\"\"");
    let args = &args[3..args.len() - 3];
    let mut args_lexer = Lexer::from_slice(args, args_line);
    token_iter.expect(Token::into_close_paren)?;
    Ok(hhbc::hhas_attribute::HhasAttribute {
        name,
        arguments: assemble_user_attr_args(alloc, &mut args_lexer)?,
    })
}

/// Printed as follows (print_attributes in bcp)
/// "v:args.len:{args}" where args are typed values
fn assemble_user_attr_args<'arena, 'a>(
    alloc: &'arena Bump,
    token_iter: &mut Lexer<'a>,
) -> Result<Slice<'arena, hhbc::TypedValue<'arena>>> {
    if let hhbc::TypedValue::Vec(sl) = assemble_typed_value(alloc, token_iter)? {
        Ok(sl)
    } else {
        bail!("Malformed user_attr_args -- should be a vec")
    }
}

/// Ex: <"HH\\void" N >
/// < "user_type" "type_constraint.name" type_constraint.flags >
fn assemble_type_info<'arena, 'a>(
    alloc: &'arena Bump,
    token_iter: &mut Lexer<'a>,
) -> Result<Maybe<hhbc::hhas_type::HhasTypeInfo<'arena>>> {
    token_iter.expect(Token::into_lt)?;
    let user_type = token_iter.expect(Token::into_str_literal)?;
    let user_type = escaper::unquote_slice(user_type);
    let user_type = escaper::unescape_bytes(user_type)?;
    let user_type = Maybe::Just(Str::new_slice(alloc, &user_type));
    let type_cons_name = match token_iter.peek() {
        Some(Token::StrLiteral(_, _)) => todo!(), // Don't know how to get type_constraint.name yet; only know about N (no name)
        _ => {
            if token_iter.next_if_str(Token::is_identifier, "N") {
                Maybe::Nothing
            } else {
                todo!()
            }
        }
    };
    let type_cons_flags = match token_iter.peek() {
        Some(Token::Gt(_)) => hhvm_types_ffi::ffi::TypeConstraintFlags::NoFlags,
        _ => todo!(), // Don't know how to do type flags yet
    };
    token_iter.expect(Token::into_gt)?;
    let cons = hhbc::hhas_type::Constraint::make(type_cons_name, type_cons_flags);
    Ok(Maybe::Just(hhbc::hhas_type::HhasTypeInfo::make(
        user_type, cons,
    )))
}

/// ((a, )*a) | () where a is a param
fn assemble_params<'arena, 'a>(
    alloc: &'arena Bump,
    token_iter: &mut Lexer<'a>,
    decl_map: &mut HashMap<&'a [u8], u32>,
) -> Result<Slice<'arena, hhbc::hhas_param::HhasParam<'arena>>> {
    token_iter.expect(Token::into_open_paren)?;
    let mut params = Vec::new();
    while !token_iter.peek_if(Token::is_close_paren) {
        params.push(assemble_param(alloc, token_iter, decl_map)?);
        if !token_iter.peek_if(Token::is_close_paren) {
            token_iter.expect(Token::into_comma)?;
        }
    }
    token_iter.expect(Token::into_close_paren)?;
    Ok(Slice::from_vec(alloc, params))
}

/// a: <user_attributes>? inout? readonly? ...?<type_info>?name (= default_value)?
fn assemble_param<'arena, 'a>(
    alloc: &'arena Bump,
    token_iter: &mut Lexer<'a>,
    decl_map: &mut HashMap<&'a [u8], u32>,
) -> Result<hhbc::hhas_param::HhasParam<'arena>> {
    let mut ua_vec = Vec::new();
    let user_attributes = {
        if token_iter.peek_if(Token::is_open_bracket) {
            token_iter.expect(Token::into_open_bracket)?;
            while !token_iter.peek_if(Token::is_close_bracket) {
                ua_vec.push(assemble_user_attr(alloc, token_iter)?);
            }
            token_iter.expect(Token::into_close_bracket)?;
        }
        Slice::from_vec(alloc, ua_vec)
    };
    let is_inout = token_iter.next_if_str(Token::is_identifier, "inout");
    let is_readonly = token_iter.next_if_str(Token::is_identifier, "readonly");
    let is_variadic = token_iter.next_if(Token::is_variadic);
    let type_info = if token_iter.peek_if(Token::is_lt) {
        assemble_type_info(alloc, token_iter)? // Unlike user_attrs, consumes <> too
    } else {
        Maybe::Nothing
    };
    let name = token_iter.expect(Token::into_variable)?;
    decl_map.insert(name, decl_map.len() as u32);
    let name = Str::new_slice(alloc, name);
    let default_value = if token_iter.peek_if(Token::is_equal) {
        todo!() // is type Maybe<Pair<Label, Str<'arena>>>
    } else {
        Maybe::Nothing
    };
    Ok(hhbc::hhas_param::HhasParam {
        name,
        is_variadic,
        is_inout,
        is_readonly,
        user_attributes,
        type_info,
        default_value,
    })
}

/// { (.doc ...)? (.ismemoizewrapper)? (.ismemoizewrapperlsb)? (.numiters)? (.declvars)? (instructions)* }
fn assemble_body<'arena, 'a>(
    alloc: &'arena Bump,
    token_iter: &mut Lexer<'a>,
    decl_map: &mut HashMap<&'a [u8], u32>,
) -> Result<hhbc::hhas_body::HhasBody<'arena>> {
    let mut instrs = Vec::new();
    let mut decl_vars = Slice::default();
    let mut num_iters = 0;
    // For now we don't parse params, so will just have decl_vars (might need to move this later)
    token_iter.expect(Token::into_open_curly)?;
    // In body, before instructions, are 5 possible constructs:
    // only .declvars can be declared more than once
    while token_iter.peek_if(Token::is_decl) {
        if token_iter.peek_if_str(Token::is_decl, ".doc")
            || token_iter.peek_if_str(Token::is_decl, ".ismemoizewrapper")
            || token_iter.peek_if_str(Token::is_decl, ".ismemoizewrapperlsb")
        {
            todo!("Have yet to do: {}", token_iter.peek().unwrap())
        } else if token_iter.peek_if_str(Token::is_decl, ".numiters") {
            if num_iters > 0 {
                bail!("Cannot have more than one .numiters per function body"); // Because only printed once in print.rs
            }
            num_iters = assemble_numiters(token_iter)?;
        } else if token_iter.peek_if_str(Token::is_decl, ".declvars") {
            if !decl_vars.is_empty() {
                bail!("Cannot have more than one .declvars per function body");
            }
            decl_vars = assemble_decl_vars(alloc, token_iter, decl_map)?;
        } else {
            break;
        }
    }
    // And maybe coeffects, not sure what that is yet
    // tcb_count tells how many TryCatchBegins there are that are still unclosed
    // we only stop parsing instructions once we see a is_close_curly and tcb_count is 0
    let mut tcb_count = 0;
    while tcb_count > 0 || !token_iter.peek_if(Token::is_close_curly) {
        instrs.push(assemble_instr(alloc, token_iter, decl_map, &mut tcb_count)?);
    }
    token_iter.expect(Token::into_close_curly)?;
    let tr = hhbc::hhas_body::HhasBody {
        body_instrs: Slice::from_vec(alloc, instrs),
        decl_vars,
        num_iters,
        ..Default::default()
    };
    Ok(tr)
}

/// Expects .numiters #+;
fn assemble_numiters<'a>(token_iter: &mut Lexer<'a>) -> Result<usize> {
    token_iter.expect_is_str(Token::into_decl, ".numiters")?;
    let num = token_iter.expect_and_get_number()?;
    token_iter.expect(Token::into_semicolon)?;
    Ok(num)
}

/// Expects .declvars ($x)+;
fn assemble_decl_vars<'arena, 'a>(
    alloc: &'arena Bump,
    token_iter: &mut Lexer<'a>,
    decl_map: &mut HashMap<&'a [u8], u32>,
) -> Result<Slice<'arena, Str<'arena>>> {
    token_iter.expect_is_str(Token::into_decl, ".declvars")?;
    let mut var_names = Vec::new();
    while !token_iter.peek_if(Token::is_semicolon) {
        let var_nm = token_iter.expect(Token::into_variable)?;
        decl_map.insert(var_nm, decl_map.len().try_into().unwrap());
        var_names.push(Str::new_slice(alloc, var_nm));
    }
    token_iter.expect(Token::into_semicolon)?;
    Ok(Slice::from_vec(alloc, var_names))
}

/// Opcodes and Pseudos
fn assemble_instr<'arena, 'a>(
    alloc: &'arena Bump,
    token_iter: &mut Lexer<'a>,
    decl_map: &mut HashMap<&'a [u8], u32>,
    tcb_count: &mut usize, // Increase this when get TryCatchBegin, decrease when TryCatchEnd
) -> Result<hhbc::Instruct<'arena>> {
    let label_reg = regex!(r"^((DV|L)[0-9]+)$");
    if let Some(mut sl_lexer) = token_iter.fetch_until_newline() {
        if sl_lexer.peek_if(Token::is_decl) {
            // Not all pseudos are decls, but all instruction decls are pseudos
            if sl_lexer.peek_if_str(Token::is_decl, ".srcloc") {
                Ok(hhbc::Instruct::Pseudo(hhbc::Pseudo::SrcLoc(
                    assemble_srcloc(&mut sl_lexer)?,
                )))
            } else if sl_lexer.next_if_str(Token::is_decl, ".try") {
                sl_lexer.expect(Token::into_open_curly)?;
                *tcb_count += 1;
                Ok(hhbc::Instruct::Pseudo(hhbc::Pseudo::TryCatchBegin))
            } else {
                todo!("{}", sl_lexer.next().unwrap());
            }
        } else if sl_lexer.next_if(Token::is_close_curly) {
            if sl_lexer.next_if_str(Token::is_decl, ".catch") {
                // Is a TCM
                sl_lexer.expect(Token::into_open_curly)?;
                Ok(hhbc::Instruct::Pseudo(hhbc::Pseudo::TryCatchMiddle))
            } else {
                // Is a TCE
                debug_assert!(*tcb_count > 0);
                *tcb_count -= 1;
                Ok(hhbc::Instruct::Pseudo(hhbc::Pseudo::TryCatchEnd))
            }
        } else if sl_lexer.peek_if(Token::is_identifier) {
            if let Some(tok) = sl_lexer.peek() {
                match tok.as_bytes() {
                    label if label_reg.is_match(label) => Ok(hhbc::Instruct::Pseudo(
                        hhbc::Pseudo::Label(assemble_label(&mut sl_lexer, true)?),
                    )),
                    b"Add" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::Add,
                        "Add",
                    ),
                    b"Sub" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::Sub,
                        "Sub",
                    ),
                    b"Mul" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::Mul,
                        "Mul",
                    ),
                    b"AddO" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::AddO,
                        "AddO",
                    ),
                    b"SubO" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::SubO,
                        "SubO",
                    ),
                    b"MulO" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::MulO,
                        "MulO",
                    ),
                    b"Pow" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::Pow,
                        "Pow",
                    ),
                    b"Not" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::Not,
                        "Not",
                    ),
                    b"NSame" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::NSame,
                        "NSame",
                    ),
                    b"Eq" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::Eq,
                        "Eq",
                    ),
                    b"Neq" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::Neq,
                        "Neq",
                    ),
                    b"Lte" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::Lte,
                        "Lte",
                    ),
                    b"Gt" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::Gt,
                        "Gt",
                    ),
                    b"Gte" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::Gte,
                        "Gte",
                    ),
                    b"Cmp" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::Cmp,
                        "Cmp",
                    ),
                    b"BitAnd" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::BitAnd,
                        "BitAnd",
                    ),
                    b"BitOr" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::BitOr,
                        "BitOr",
                    ),
                    b"BitXor" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::BitXor,
                        "BitXor",
                    ),
                    b"BitNot" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::BitNot,
                        "BitNot",
                    ),
                    b"Shl" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::Shl,
                        "Shl",
                    ),
                    b"Shr" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::Shr,
                        "Shr",
                    ),
                    b"CastBool" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::CastBool,
                        "CastBool",
                    ),
                    b"CastInt" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::CastInt,
                        "CastInt",
                    ),
                    b"CastDouble" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::CastDouble,
                        "CastDouble",
                    ),
                    b"CastString" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::CastString,
                        "CastString",
                    ),
                    b"CastDict" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::CastDict,
                        "CastDict",
                    ),
                    b"CastKeyset" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::CastKeyset,
                        "CastKeyset",
                    ),
                    b"CastVec" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::CastVec,
                        "CastVec",
                    ),
                    b"DblAsBits" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::DblAsBits,
                        "DblAsBits",
                    ),
                    b"InstanceOf" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::InstanceOf,
                        "InstanceOf",
                    ),

                    b"Print" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::Print,
                        "Print",
                    ),
                    b"Div" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::Div,
                        "Div",
                    ),
                    b"Dir" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::Dir,
                        "Dir",
                    ),
                    b"PopC" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::PopC,
                        "PopC",
                    ),
                    b"Concat" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::Concat,
                        "Concat",
                    ),
                    b"ClassGetC" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::ClassGetC,
                        "ClassGetC",
                    ),
                    b"Null" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::Null,
                        "Null",
                    ),
                    b"RetC" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::RetC,
                        "RetC",
                    ),
                    b"Lt" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::Lt,
                        "Lt",
                    ),
                    b"Mod" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::Mod,
                        "Mod",
                    ),
                    b"Exit" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::Exit,
                        "Exit",
                    ),
                    b"Same" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::Same,
                        "Same",
                    ),
                    b"NullUninit" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::NullUninit,
                        "NullUninit",
                    ),
                    b"String" => assemble_string_opcode(alloc, &mut sl_lexer),
                    b"Int" => assemble_int_opcode(alloc, &mut sl_lexer),
                    b"Double" => assemble_double_opcode(alloc, &mut sl_lexer),
                    b"False" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::False,
                        "False",
                    ),
                    b"Dict" => assemble_adata_id_carrying_instr(
                        alloc,
                        &mut sl_lexer,
                        hhbc::Opcode::Dict,
                        "Dict",
                    ),
                    b"Vec" => assemble_adata_id_carrying_instr(
                        alloc,
                        &mut sl_lexer,
                        hhbc::Opcode::Vec,
                        "Vec",
                    ),
                    b"True" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::True,
                        "True",
                    ),
                    b"VerifyOutType" => assemble_local_carrying_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        decl_map,
                        hhbc::Opcode::VerifyOutType,
                        "VerifyOutType",
                    ),
                    b"VerifyParamType" => assemble_local_carrying_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        decl_map,
                        hhbc::Opcode::VerifyParamType,
                        "VerifyParamType",
                    ),
                    b"VerifyParamTypeTS" => assemble_local_carrying_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        decl_map,
                        hhbc::Opcode::VerifyParamTypeTS,
                        "VerifyParamTypeTS",
                    ),
                    b"SetL" => assemble_local_carrying_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        decl_map,
                        hhbc::Opcode::SetL,
                        "SetL",
                    ),
                    b"UnsetL" => assemble_local_carrying_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        decl_map,
                        hhbc::Opcode::UnsetL,
                        "UnsetL",
                    ),
                    b"PopL" => assemble_local_carrying_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        decl_map,
                        hhbc::Opcode::PopL,
                        "PopL",
                    ),
                    b"ClsCnsL" => assemble_local_carrying_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        decl_map,
                        hhbc::Opcode::ClsCnsL,
                        "ClsCnsL",
                    ),
                    b"CGetL2" => assemble_local_carrying_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        decl_map,
                        hhbc::Opcode::CGetL2,
                        "CGetL2",
                    ),
                    b"CGetL" => assemble_local_carrying_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        decl_map,
                        hhbc::Opcode::CGetL,
                        "CGetL",
                    ),
                    b"CUGetL" => assemble_local_carrying_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        decl_map,
                        hhbc::Opcode::CUGetL,
                        "CUGetL",
                    ),
                    b"PushL" => assemble_local_carrying_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        decl_map,
                        hhbc::Opcode::PushL,
                        "PushL",
                    ),
                    b"GetMemoKeyL" => assemble_local_carrying_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        decl_map,
                        hhbc::Opcode::GetMemoKeyL,
                        "GetMemoKeyL",
                    ),
                    b"IssetL" => assemble_local_carrying_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        decl_map,
                        hhbc::Opcode::IssetL,
                        "IssetL",
                    ),
                    b"CGetQuietL" => assemble_local_carrying_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        decl_map,
                        hhbc::Opcode::CGetQuietL,
                        "CGetQuietL",
                    ),
                    b"IsUnsetL" => assemble_local_carrying_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        decl_map,
                        hhbc::Opcode::IsUnsetL,
                        "IsUnsetL",
                    ),
                    b"JmpZ" => {
                        assemble_jump_opcode_instr(alloc, &mut sl_lexer, hhbc::Opcode::JmpZ, "JmpZ")
                    }
                    b"Jmp" => {
                        assemble_jump_opcode_instr(alloc, &mut sl_lexer, hhbc::Opcode::Jmp, "Jmp")
                    }
                    b"JmpNZ" => assemble_jump_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        hhbc::Opcode::JmpNZ,
                        "JmpNZ",
                    ),
                    b"Enter" => assemble_jump_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        hhbc::Opcode::Enter,
                        "Enter",
                    ),
                    b"FCallFuncD" => assemble_fcallfuncd_opcode(alloc, &mut sl_lexer),
                    b"IncDecL" => assemble_incdecl_opcode(alloc, &mut sl_lexer, decl_map),
                    b"IsTypeStructC" => assemble_is_type_struct_c(alloc, &mut sl_lexer),
                    b"IsTypeC" => assemble_is_type_c(alloc, &mut sl_lexer),
                    b"IsTypeL" => assemble_is_type_l(alloc, &mut sl_lexer, decl_map),
                    b"IterFree" => assemble_iter_free(alloc, &mut sl_lexer),
                    b"IterInit" => assemble_iter_init_iter_next(
                        alloc,
                        &mut sl_lexer,
                        decl_map,
                        hhbc::Opcode::IterInit,
                        "IterInit",
                    ),
                    b"IterNext" => assemble_iter_init_iter_next(
                        alloc,
                        &mut sl_lexer,
                        decl_map,
                        hhbc::Opcode::IterNext,
                        "IterNext",
                    ),
                    b"BaseGC" => {
                        assemble_base_gc_or_c(alloc, &mut sl_lexer, hhbc::Opcode::BaseGC, "BaseGC")
                    }
                    b"BaseGL" => assemble_base_gl(alloc, &mut sl_lexer, decl_map),
                    b"BaseSC" => assemble_base_sc(alloc, &mut sl_lexer),
                    b"BaseL" => assemble_base_l(alloc, &mut sl_lexer, decl_map),
                    b"BaseC" => {
                        assemble_base_gc_or_c(alloc, &mut sl_lexer, hhbc::Opcode::BaseC, "BaseC")
                    }
                    b"BaseH" => assemble_single_opcode_instr(
                        alloc,
                        &mut sl_lexer,
                        || hhbc::Opcode::BaseH,
                        "BaseH",
                    ),
                    b"Dim" => assemble_dim(alloc, &mut sl_lexer, decl_map),
                    b"QueryM" => assemble_query_m(alloc, &mut sl_lexer, decl_map),
                    b"SetM" => assemble_set_unset_m(
                        alloc,
                        &mut sl_lexer,
                        decl_map,
                        hhbc::Opcode::SetM,
                        "SetM",
                    ),
                    b"IncDecM" => assemble_inc_dec_m(alloc, &mut sl_lexer, decl_map),
                    b"SetOpM" => assemble_set_op_m(alloc, &mut sl_lexer, decl_map),
                    b"UnsetM" => assemble_set_unset_m(
                        alloc,
                        &mut sl_lexer,
                        decl_map,
                        hhbc::Opcode::UnsetM,
                        "UnsetM",
                    ),
                    b"ClsCns" => assemble_cls_cns(alloc, &mut sl_lexer),
                    b"RetM" => assemble_retm_opcode_instr(alloc, &mut sl_lexer),
                    _ => todo!("assembling instrs: {}", tok),
                }
            } else {
                bail!("Something wrong in assemble_instr") // Shouldn't happen, we peek_if so there must be something
            }
        } else {
            bail!(
                "Function body line that's neither decl or identifier: {}",
                sl_lexer.next().unwrap() // Know there is something here because fetch_until_new_line won't return empty
            );
        }
    } else {
        bail!("Expected an additional instruction in body, reached EOF");
    }
}

/// .srcloc #:#,#:#;
fn assemble_srcloc<'a>(token_iter: &mut Lexer<'a>) -> Result<hhbc::SrcLoc> {
    token_iter.expect_is_str(Token::into_decl, ".srcloc")?;
    let tr = hhbc::SrcLoc {
        line_begin: token_iter.expect_and_get_number()?,
        col_begin: {
            token_iter.expect(Token::into_colon)?;
            token_iter.expect_and_get_number()?
        },
        line_end: {
            token_iter.expect(Token::into_comma)?;
            token_iter.expect_and_get_number()?
        },
        col_end: {
            token_iter.expect(Token::into_colon)?;
            token_iter.expect_and_get_number()?
        },
    };
    token_iter.expect(Token::into_semicolon)?;
    token_iter.expect_end()?;
    Ok(tr)
}

/// <(fcallargflag)*>
fn assemble_fcallargsflags<'a>(token_iter: &mut Lexer<'a>) -> Result<hhbc::FCallArgsFlags> {
    let mut flags = hhbc::FCallArgsFlags::FCANone;
    token_iter.expect(Token::into_lt)?;
    while !token_iter.peek_if(Token::is_gt) {
        let flg = token_iter.expect(Token::into_identifier)?;
        match flg {
            b"HasUnpack" => flags.add(hhbc::FCallArgsFlags::HasUnpack),
            b"HasGenerics" => flags.add(hhbc::FCallArgsFlags::HasGenerics),
            b"LockWhileUnwinding" => flags.add(hhbc::FCallArgsFlags::LockWhileUnwinding),
            b"SkipRepack" => flags.add(hhbc::FCallArgsFlags::SkipRepack),
            b"SkipCoeffectsCheck" => flags.add(hhbc::FCallArgsFlags::SkipCoeffectsCheck),
            b"EnforceMutableReturn" => flags.add(hhbc::FCallArgsFlags::EnforceMutableReturn),
            b"EnforceReadonlyThis" => flags.add(hhbc::FCallArgsFlags::EnforceReadonlyThis),
            b"ExplicitContext" => flags.add(hhbc::FCallArgsFlags::ExplicitContext),
            b"HasInOut" => flags.add(hhbc::FCallArgsFlags::HasInOut),
            b"EnforceInOut" => flags.add(hhbc::FCallArgsFlags::EnforceInOut),
            b"EnforceReadonly" => flags.add(hhbc::FCallArgsFlags::EnforceReadonly),
            b"HasAsyncEagerOffset" => flags.add(hhbc::FCallArgsFlags::HasAsyncEagerOffset),
            b"NumArgsStart" => flags.add(hhbc::FCallArgsFlags::NumArgsStart),
            _ => bail!("Unrecognized FCallArgsFlags"),
        }
    }
    token_iter.expect(Token::into_gt)?;
    Ok(flags)
}

/// "(0|1)*"
fn assemble_inouts_or_readonly<'arena, 'a>(
    alloc: &'arena Bump,
    token_iter: &mut Lexer<'a>,
) -> Result<Slice<'arena, bool>> {
    let literal = token_iter.expect(Token::into_str_literal)?; //
    debug_assert!(literal[0] == b'"' && literal[literal.len() - 1] == b'"');
    let tr: Result<Vec<bool>, _> = literal[1..literal.len() - 1] //trims the outer "", which are guaranteed b/c of str token
        .iter()
        .map(|c| {
            Ok(if *c == b'0' {
                false
            } else if *c == b'1' {
                true
            } else {
                bail!("Non 0/1 character in inouts/readonlys")
            })
        })
        .collect();
    Ok(Slice::from_vec(alloc, tr?))
}

/// -
fn assemble_async_eager_target<'a>(token_iter: &mut Lexer<'a>) -> Result<Option<hhbc::Label>> {
    if token_iter.next_if(Token::is_dash) {
        // Not sure how this appears otherwise. If it's just a dash means no async eager target
        Ok(None)
    } else {
        todo!()
    }
}

/// Just a string literal
fn assemble_fcall_context<'a, 'arena>(
    alloc: &'arena Bump,
    token_iter: &mut Lexer<'a>,
) -> Result<Str<'arena>> {
    let st = token_iter.expect(Token::into_str_literal)?;
    debug_assert!(st[0] == b'"' && st[st.len() - 1] == b'"');
    Ok(Str::new_slice(alloc, &st[1..st.len() - 1])) // if not hugged by "", won't pass into_str_literal
}

/// FCallFuncD <(fcargflags)*> numargs numrets inouts readonly async_eager_target context name
fn assemble_fcallfuncd_opcode<'arena, 'a>(
    alloc: &'arena Bump,
    token_iter: &mut Lexer<'a>,
) -> Result<hhbc::Instruct<'arena>> {
    token_iter.expect_is_str(Token::into_identifier, "FCallFuncD")?;
    let fcargflags = assemble_fcallargsflags(token_iter)?;
    let num_args: u32 = token_iter.expect_and_get_number()?;
    let num_rets: u32 = token_iter.expect_and_get_number()?;
    let inouts: Slice<'arena, bool> = assemble_inouts_or_readonly(alloc, token_iter)?;
    let readonly: Slice<'arena, bool> = assemble_inouts_or_readonly(alloc, token_iter)?;
    let async_eager_target: Option<hhbc::Label> = assemble_async_eager_target(token_iter)?;
    let context: Str<'arena> = assemble_fcall_context(alloc, token_iter)?;
    let fcargs = hhbc::FCallArgs::new(
        fcargflags,
        num_rets,
        num_args,
        inouts,
        readonly,
        async_eager_target,
        None, // I did this b/c FCA's new expects a context that is a Option<'arena &str>, not sure how to make it
    );
    let fcargs = hhbc::FCallArgs { context, ..fcargs };
    // Set fname (which is printed in quotes)
    let func_name = escaper::unescape_literal_bytes_into_vec_bytes(escaper::unquote_slice(
        token_iter.expect(Token::into_str_literal)?,
    ))?;
    let func_name = hhbc::FunctionName::new(Str::new_slice(alloc, &func_name));
    token_iter.expect_end()?;
    Ok(hhbc::Instruct::Opcode(hhbc::Opcode::FCallFuncD(
        fcargs, func_name,
    )))
}

fn assemble_is_type_op(token_iter: &mut Lexer<'_>) -> Result<hhbc::IsTypeOp> {
    match token_iter.expect(Token::into_identifier)? {
        b"Null" => Ok(hhbc::IsTypeOp::Null),
        b"Bool" => Ok(hhbc::IsTypeOp::Bool),
        b"Int" => Ok(hhbc::IsTypeOp::Int),
        b"Dbl" => Ok(hhbc::IsTypeOp::Dbl),
        b"Str" => Ok(hhbc::IsTypeOp::Str),
        b"Obj" => Ok(hhbc::IsTypeOp::Obj),
        b"Res" => Ok(hhbc::IsTypeOp::Res),
        b"Scalar" => Ok(hhbc::IsTypeOp::Scalar),
        b"Keyset" => Ok(hhbc::IsTypeOp::Keyset),
        b"Dict" => Ok(hhbc::IsTypeOp::Vec),
        b"ArrLike" => Ok(hhbc::IsTypeOp::ArrLike),
        b"ClsMeth" => Ok(hhbc::IsTypeOp::ClsMeth),
        b"Func" => Ok(hhbc::IsTypeOp::Func),
        b"LegacyArrLike" => Ok(hhbc::IsTypeOp::LegacyArrLike),
        b"Class" => Ok(hhbc::IsTypeOp::Class),
        ito => bail!("Unknown IsTypeOp: {:?}", ito),
    }
}

/// Ex: IterFree 0
fn assemble_iter_free<'arena>(
    _alloc: &'arena Bump,
    token_iter: &mut Lexer<'_>,
) -> Result<hhbc::Instruct<'arena>> {
    token_iter.expect_is_str(Token::into_identifier, "IterFree")?;
    let iter_id = hhbc::IterId {
        idx: token_iter.expect_and_get_number()?,
    };
    Ok(hhbc::Instruct::Opcode(hhbc::Opcode::IterFree(iter_id)))
}

/// Ex:
/// IterInit: IterInit 0 NK V:$v L0 (IterInit IterArgs Label)
/// IterNext: IterNext 0 NK V:$v L1 (IterInit IterArgs Label)
fn assemble_iter_init_iter_next<
    'arena,
    F: FnOnce(hhbc::IterArgs, hhbc::Label) -> hhbc::Opcode<'arena>,
>(
    _alloc: &'arena Bump,
    token_iter: &mut Lexer<'_>,
    decl_map: &HashMap<&'_ [u8], u32>,
    op_con: F,
    op_str: &str,
) -> Result<hhbc::Instruct<'arena>> {
    token_iter.expect_is_str(Token::into_identifier, op_str)?;
    let iter_args = assemble_iter_args(token_iter, decl_map)?;
    let lbl = assemble_label(token_iter, false)?;
    Ok(hhbc::Instruct::Opcode(op_con(iter_args, lbl)))
}

/// IterArg { iter_id: IterId (~u32), key_id: Local, val_id: Local}
/// Ex: 0 NK V:$v
fn assemble_iter_args(
    token_iter: &mut Lexer<'_>,
    decl_map: &HashMap<&'_ [u8], u32>,
) -> Result<hhbc::IterArgs> {
    let idx: u32 = token_iter.expect_and_get_number()?;
    let key_id: hhbc::Local = match token_iter.expect(Token::into_identifier)? {
        b"NK" => hhbc::Local::INVALID,
        b"K" => {
            token_iter.expect(Token::into_colon)?;
            assemble_var_to_local(token_iter, decl_map)?
        }
        o => bail!("Invalid key_id given as iter args to IterArg: {:?}", o),
    };
    token_iter.expect_is_str(Token::into_identifier, "V")?;
    token_iter.expect(Token::into_colon)?;
    Ok(hhbc::IterArgs {
        iter_id: hhbc::IterId { idx },
        key_id,
        val_id: assemble_var_to_local(token_iter, decl_map)?,
    })
}

/// Ex: IsTypeL $a Obj
fn assemble_is_type_l<'arena>(
    _alloc: &'arena Bump,
    token_iter: &mut Lexer<'_>,
    decl_map: &HashMap<&'_ [u8], u32>,
) -> Result<hhbc::Instruct<'arena>> {
    token_iter.expect_is_str(Token::into_identifier, "IsTypeL")?;
    let lcl = assemble_var_to_local(token_iter, decl_map)?;
    let type_op = assemble_is_type_op(token_iter)?;
    Ok(hhbc::Instruct::Opcode(hhbc::Opcode::IsTypeL(lcl, type_op)))
}

/// Ex: IsTypeC Obj
fn assemble_is_type_c<'arena>(
    _alloc: &'arena Bump,
    token_iter: &mut Lexer<'_>,
) -> Result<hhbc::Instruct<'arena>> {
    token_iter.expect_is_str(Token::into_identifier, "IsTypeC")?;
    let type_op = assemble_is_type_op(token_iter)?;
    Ok(hhbc::Instruct::Opcode(hhbc::Opcode::IsTypeC(type_op)))
}

/// Ex: ClsCns "CLASSNAME"
fn assemble_cls_cns<'arena>(
    alloc: &'arena Bump,
    token_iter: &mut Lexer<'_>,
) -> Result<hhbc::Instruct<'arena>> {
    token_iter.expect_is_str(Token::into_identifier, "ClsCns")?;
    let nm = token_iter.expect(Token::into_identifier)?;
    Ok(hhbc::Instruct::Opcode(hhbc::Opcode::ClsCns(
        hhbc::ConstName::new(Str::new_slice(alloc, nm)),
    )))
}

/// IsTypeStructC (Resolve|DontResolve)
fn assemble_is_type_struct_c<'arena>(
    _alloc: &'arena Bump,
    token_iter: &mut Lexer<'_>,
) -> Result<hhbc::Instruct<'arena>> {
    token_iter.expect_is_str(Token::into_identifier, "IsTypeStructC")?;
    let res_op = token_iter.expect(Token::into_identifier)?;
    match res_op {
        b"Resolve" => Ok(hhbc::Instruct::Opcode(hhbc::Opcode::IsTypeStructC(
            hhbc::TypeStructResolveOp::Resolve,
        ))),
        b"DontResolve" => Ok(hhbc::Instruct::Opcode(hhbc::Opcode::IsTypeStructC(
            hhbc::TypeStructResolveOp::DontResolve,
        ))),
        _ => bail!(
            "Unknown TypeStructResolveOp passed to TypeStructC instr: {:?}",
            res_op
        ),
    }
}

/// StackIndex : u32
fn assemble_stack_index(token_iter: &mut Lexer<'_>) -> Result<hhbc::StackIndex> {
    token_iter.expect_and_get_number()
}

/// struct Propname<'arena>(Str<'arena>)
fn assemble_prop_name<'arena>(
    _alloc: &'arena Bump,
    _token_iter: &mut Lexer<'_>,
) -> Result<hhbc::PropName<'arena>> {
    todo!()
}

fn assemble_mop_mode(token_iter: &mut Lexer<'_>) -> Result<hhbc::MOpMode> {
    match token_iter.expect(Token::into_identifier)? {
        b"None" => Ok(hhbc::MOpMode::None),
        b"Warn" => Ok(hhbc::MOpMode::Warn),
        b"Define" => Ok(hhbc::MOpMode::Define),
        b"Unset" => Ok(hhbc::MOpMode::Unset),
        b"Inout" => Ok(hhbc::MOpMode::InOut),
        mop => bail!("Expected a MOpMode but got: {:?}", mop),
    }
}

fn assemble_readonly_op(token_iter: &mut Lexer<'_>) -> Result<hhbc::ReadonlyOp> {
    match token_iter.expect(Token::into_identifier)? {
        b"Any" => Ok(hhbc::ReadonlyOp::Any),
        b"Readonly" => Ok(hhbc::ReadonlyOp::Readonly),
        b"Mutable" => Ok(hhbc::ReadonlyOp::Mutable),
        b"CheckROCOW" => Ok(hhbc::ReadonlyOp::CheckROCOW),
        b"CheckMutROCOW" => Ok(hhbc::ReadonlyOp::CheckMutROCOW),
        rop => {
            bail!("Expected a ReadonlyOp but got: {:?}", rop)
        }
    }
}

/// EC: stackIndex readOnlyOp | EL: local readOnlyOp | ET: string readOnlyOp | EI: int readOnlyOp
/// PC: stackIndex readOnlyOp | PL: local readOnlyOp | PT: propName readOnlyOp | QT: propName readOnlyOp
fn assemble_member_key<'arena>(
    alloc: &'arena Bump,
    token_iter: &mut Lexer<'_>,
    decl_map: &HashMap<&'_ [u8], u32>,
) -> Result<hhbc::MemberKey<'arena>> {
    match token_iter.expect(Token::into_identifier)? {
        b"EC" => {
            token_iter.expect(Token::into_colon)?;
            Ok(hhbc::MemberKey::EC(
                assemble_stack_index(token_iter)?,
                assemble_readonly_op(token_iter)?,
            ))
        }
        b"EL" => {
            token_iter.expect(Token::into_colon)?;
            Ok(hhbc::MemberKey::EL(
                assemble_var_to_local(token_iter, decl_map)?,
                assemble_readonly_op(token_iter)?,
            ))
        }
        b"ET" => {
            token_iter.expect(Token::into_colon)?;
            Ok(hhbc::MemberKey::ET(
                Str::new_slice(
                    alloc,
                    &escaper::unescape_literal_bytes_into_vec_bytes(
                        // In bp, print_quoted_str also escapes the string
                        escaper::unquote_slice(token_iter.expect(Token::into_str_literal)?),
                    )?,
                ),
                assemble_readonly_op(token_iter)?,
            ))
        }
        b"EI" => {
            token_iter.expect(Token::into_colon)?;
            Ok(hhbc::MemberKey::EI(
                token_iter.expect_and_get_number()?,
                assemble_readonly_op(token_iter)?,
            ))
        }
        b"PC" => {
            token_iter.expect(Token::into_colon)?;
            Ok(hhbc::MemberKey::PC(
                assemble_stack_index(token_iter)?,
                assemble_readonly_op(token_iter)?,
            ))
        }
        b"PL" => {
            token_iter.expect(Token::into_colon)?;
            Ok(hhbc::MemberKey::PL(
                assemble_var_to_local(token_iter, decl_map)?,
                assemble_readonly_op(token_iter)?,
            ))
        }
        b"PT" => {
            token_iter.expect(Token::into_colon)?;
            Ok(hhbc::MemberKey::PT(
                assemble_prop_name(alloc, token_iter)?,
                assemble_readonly_op(token_iter)?,
            ))
        }
        b"QT" => {
            token_iter.expect(Token::into_colon)?;
            Ok(hhbc::MemberKey::QT(
                assemble_prop_name(alloc, token_iter)?,
                assemble_readonly_op(token_iter)?,
            ))
        }
        b"W" => Ok(hhbc::MemberKey::W),
        mk => {
            bail!("Expected a MemberKey but got: {:?}", mk)
        }
    }
}

/// Dim MOpMode MemberKey
fn assemble_dim<'arena>(
    alloc: &'arena Bump,
    token_iter: &mut Lexer<'_>,
    decl_map: &HashMap<&'_ [u8], u32>,
) -> Result<hhbc::Instruct<'arena>> {
    token_iter.expect_is_str(Token::into_identifier, "Dim")?;
    Ok(hhbc::Instruct::Opcode(hhbc::Opcode::Dim(
        assemble_mop_mode(token_iter)?,
        assemble_member_key(alloc, token_iter, decl_map)?,
    )))
}

/// QueryM ...
fn assemble_query_m<'arena>(
    _alloc: &'arena Bump,
    _token_iter: &mut Lexer<'_>,
    _decl_map: &HashMap<&'_ [u8], u32>,
) -> Result<hhbc::Instruct<'arena>> {
    todo!()
}

/// (SetM|UnsetM) stackIndex MemberKey
fn assemble_set_unset_m<
    'arena,
    F: FnOnce(hhbc::StackIndex, hhbc::MemberKey<'arena>) -> hhbc::Opcode<'arena>,
>(
    alloc: &'arena Bump,
    token_iter: &mut Lexer<'_>,
    decl_map: &HashMap<&'_ [u8], u32>,
    op_con: F,
    op_str: &str,
) -> Result<hhbc::Instruct<'arena>> {
    token_iter.expect_is_str(Token::into_identifier, op_str)?;
    Ok(hhbc::Instruct::Opcode(op_con(
        assemble_stack_index(token_iter)?,
        assemble_member_key(alloc, token_iter, decl_map)?,
    )))
}

/// IncDecM stackIndex IncDecOp MemberKey
fn assemble_inc_dec_m<'arena>(
    alloc: &'arena Bump,
    token_iter: &mut Lexer<'_>,
    decl_map: &HashMap<&'_ [u8], u32>,
) -> Result<hhbc::Instruct<'arena>> {
    token_iter.expect_is_str(Token::into_identifier, "IncDecM")?;
    Ok(hhbc::Instruct::Opcode(hhbc::Opcode::IncDecM(
        assemble_stack_index(token_iter)?,
        assemble_inc_dec_op(token_iter)?,
        assemble_member_key(alloc, token_iter, decl_map)?,
    )))
}

/// SetOpM ...
fn assemble_set_op_m<'arena>(
    _alloc: &'arena Bump,
    _token_iter: &mut Lexer<'_>,
    _decl_map: &HashMap<&'_ [u8], u32>,
) -> Result<hhbc::Instruct<'arena>> {
    todo!()
}

/// BaseGL $var MOpMode
fn assemble_base_gl<'arena>(
    _alloc: &'arena Bump,
    token_iter: &mut Lexer<'_>,
    decl_map: &HashMap<&'_ [u8], u32>,
) -> Result<hhbc::Instruct<'arena>> {
    token_iter.expect_is_str(Token::into_identifier, "BaseGL")?;
    Ok(hhbc::Instruct::Opcode(hhbc::Opcode::BaseGL(
        assemble_var_to_local(token_iter, decl_map)?,
        assemble_mop_mode(token_iter)?,
    )))
}

/// BaseSC stackIndex stackIndex MOpMode ReadOnlyOp
fn assemble_base_sc<'arena>(
    _alloc: &'arena Bump,
    token_iter: &mut Lexer<'_>,
) -> Result<hhbc::Instruct<'arena>> {
    token_iter.expect_is_str(Token::into_identifier, "BaseSC")?;
    Ok(hhbc::Instruct::Opcode(hhbc::Opcode::BaseSC(
        assemble_stack_index(token_iter)?,
        assemble_stack_index(token_iter)?,
        assemble_mop_mode(token_iter)?,
        assemble_readonly_op(token_iter)?,
    )))
}

/// BaseL $var MOpMode ReadOnlyOp
fn assemble_base_l<'arena>(
    _alloc: &'arena Bump,
    token_iter: &mut Lexer<'_>,
    decl_map: &HashMap<&'_ [u8], u32>,
) -> Result<hhbc::Instruct<'arena>> {
    token_iter.expect_is_str(Token::into_identifier, "BaseL")?;
    Ok(hhbc::Instruct::Opcode(hhbc::Opcode::BaseL(
        assemble_var_to_local(token_iter, decl_map)?,
        assemble_mop_mode(token_iter)?,
        assemble_readonly_op(token_iter)?,
    )))
}

/// Ex: BaseC 0 Warn
/// BaseGC 0 Warn
fn assemble_base_gc_or_c<
    'arena,
    F: FnOnce(hhbc::StackIndex, hhbc::MOpMode) -> hhbc::Opcode<'arena>,
>(
    _alloc: &'arena Bump,
    token_iter: &mut Lexer<'_>,
    op_con: F,
    op_str: &str,
) -> Result<hhbc::Instruct<'arena>> {
    token_iter.expect_is_str(Token::into_identifier, op_str)?;
    Ok(hhbc::Instruct::Opcode(op_con(
        assemble_stack_index(token_iter)?,
        assemble_mop_mode(token_iter)?,
    )))
}

fn assemble_inc_dec_op(token_iter: &mut Lexer<'_>) -> Result<hhbc::IncDecOp> {
    match token_iter.expect(Token::into_identifier)? {
        b"PreInc" => Ok(hhbc::IncDecOp::PreInc),
        b"PostInc" => Ok(hhbc::IncDecOp::PostInc),
        b"PreDec" => Ok(hhbc::IncDecOp::PreDec),
        b"PostDec" => Ok(hhbc::IncDecOp::PostDec),
        b"PreIncO" => Ok(hhbc::IncDecOp::PreIncO),
        b"PostIncO" => Ok(hhbc::IncDecOp::PostIncO),
        b"PreDecO" => Ok(hhbc::IncDecOp::PreDecO),
        b"PostDecO" => Ok(hhbc::IncDecOp::PostDecO),

        ido => bail!("Expected a IncDecOp but got: {:?}", ido),
    }
}

/// IncDecL $var IncDecOp
fn assemble_incdecl_opcode<'arena>(
    _alloc: &'arena Bump,
    token_iter: &mut Lexer<'_>,
    decl_map: &HashMap<&'_ [u8], u32>,
) -> Result<hhbc::Instruct<'arena>> {
    token_iter.expect_is_str(Token::into_identifier, "IncDecL")?;
    let lcl = token_iter.expect(Token::into_variable)?;
    if let Some(idx) = decl_map.get(lcl) {
        let ido = assemble_inc_dec_op(token_iter)?;
        token_iter.expect_end()?;
        Ok(hhbc::Instruct::Opcode(hhbc::Opcode::IncDecL(
            hhbc::Local { idx: *idx },
            ido,
        )))
    } else {
        bail!("Unknown local var given to IncDecL instr");
    }
}

/// L#: or DV#: if needs_colon else L# or DV#
fn assemble_label<'a>(token_iter: &mut Lexer<'a>, needs_colon: bool) -> Result<hhbc::Label> {
    let mut lcl = token_iter.expect(Token::into_identifier)?;
    if needs_colon {
        token_iter.expect(Token::into_colon)?;
    }
    token_iter.expect_end()?;
    let label_reg = regex!(r"^((DV|L)[0-9]+)$");
    if label_reg.is_match(lcl) {
        if lcl[0] == b'D' {
            lcl = &lcl[2..];
        } else {
            lcl = &lcl[1..];
        }
        let lcl = std::str::from_utf8(lcl)?.parse::<u32>()?;
        Ok(hhbc::Label(lcl))
    } else {
        bail!("Unknown label");
    }
}

/// Assembles one of Enter/Jmp/JmpZ/JmpNZ
fn assemble_jump_opcode_instr<'arena, F: FnOnce(hhbc::Label) -> hhbc::Opcode<'arena>>(
    _alloc: &'arena Bump,
    token_iter: &mut Lexer<'_>,
    op_con: F,
    op_str: &str,
) -> Result<hhbc::Instruct<'arena>> {
    token_iter.expect_is_str(Token::into_identifier, op_str)?;
    let lbl = assemble_label(token_iter, false)
        .map_err(|_| anyhow!("Unknown label given to {} instr", op_str))?;
    Ok(hhbc::Instruct::Opcode(op_con(lbl)))
}

/// Returns the local (u32 idx) a var corresponds to.
/// This information is based on the position of the var in parameters of a function/.declvars
fn assemble_var_to_local(
    token_iter: &mut Lexer<'_>,
    decl_map: &HashMap<&'_ [u8], u32>,
) -> Result<hhbc::Local> {
    let lcl = token_iter.expect(Token::into_variable)?;
    if let Some(idx) = decl_map.get(lcl) {
        Ok(hhbc::Local { idx: *idx })
    } else {
        bail!("Unknown local var: {:?}", lcl);
    }
}

/// Ex: RetM 2
fn assemble_retm_opcode_instr<'arena>(
    _alloc: &'arena Bump,
    token_iter: &mut Lexer<'_>,
) -> Result<hhbc::Instruct<'arena>> {
    token_iter.expect_is_str(Token::into_identifier, "RetM")?;
    Ok(hhbc::Instruct::Opcode(hhbc::Opcode::RetM(
        token_iter.expect_and_get_number()?,
    )))
}

/// Assembles one of CGetL/Push/SetL/etc...
fn assemble_local_carrying_opcode_instr<'arena, F: FnOnce(hhbc::Local) -> hhbc::Opcode<'arena>>(
    _alloc: &'arena Bump,
    token_iter: &mut Lexer<'_>,
    decl_map: &HashMap<&'_ [u8], u32>,
    op_con: F,
    op_str: &str,
) -> Result<hhbc::Instruct<'arena>> {
    token_iter.expect_is_str(Token::into_identifier, op_str)?;
    let lcl = assemble_var_to_local(token_iter, decl_map)?;
    Ok(hhbc::Instruct::Opcode(op_con(lcl)))
}

fn assemble_single_opcode_instr<'arena, F: FnOnce() -> hhbc::Opcode<'arena>>(
    _alloc: &'arena Bump,
    token_iter: &mut Lexer<'_>,
    op_con: F, // Because of the trait bound, guaranteed only trying to build opcodes that take no arguments
    op_str: &str,
) -> Result<hhbc::Instruct<'arena>> {
    token_iter.expect_is_str(Token::into_identifier, op_str)?;
    token_iter.expect_end()?;
    Ok(hhbc::Instruct::Opcode(op_con()))
}

/// Assembles one of Dict/Keyset/Vec opcodes. Note that
/// when printing, the printer adds a `@` at the front of the AdataId; it's not part of the name
fn assemble_adata_id_carrying_instr<
    'arena,
    F: FnOnce(hhbc::AdataId<'arena>) -> hhbc::Opcode<'arena>,
>(
    alloc: &'arena Bump,
    token_iter: &mut Lexer<'_>,
    op_con: F,
    op_str: &str,
) -> Result<hhbc::Instruct<'arena>> {
    token_iter.expect_is_str(Token::into_identifier, op_str)?;
    let adata_id = token_iter.expect(Token::into_global)?;
    debug_assert!(adata_id[0] == b'@');
    let adata_id: hhbc::AdataId<'arena> = Str::new_slice(alloc, &adata_id[1..]);
    Ok(hhbc::Instruct::Opcode(op_con(adata_id)))
}

fn assemble_string_opcode<'arena>(
    alloc: &'arena Bump,
    token_iter: &mut Lexer<'_>,
) -> Result<hhbc::Instruct<'arena>> {
    token_iter.expect_is_str(Token::into_identifier, "String")?;
    let st_data = token_iter.expect(Token::into_str_literal)?;
    let st_data =
        escaper::unescape_literal_bytes_into_vec_bytes(escaper::unquote_slice(st_data.as_bytes()))?;
    token_iter.expect_end()?;
    Ok(hhbc::Instruct::Opcode(hhbc::Opcode::String(
        Str::new_slice(alloc, &st_data),
    )))
}

fn assemble_int_opcode<'arena>(
    _alloc: &'arena Bump,
    token_iter: &mut Lexer<'_>,
) -> Result<hhbc::Instruct<'arena>> {
    token_iter.expect_is_str(Token::into_identifier, "Int")?;
    let num: i64 = token_iter.expect_and_get_number()?;
    token_iter.expect_end()?;
    Ok(hhbc::Instruct::Opcode(hhbc::Opcode::Int(num)))
}

fn assemble_double_opcode<'arena>(
    _alloc: &'arena Bump,
    token_iter: &mut Lexer<'_>,
) -> Result<hhbc::Instruct<'arena>> {
    token_iter.expect_is_str(Token::into_identifier, "Double")?;
    let num: f64 = token_iter.expect_and_get_number()?;
    token_iter.expect_end()?;
    Ok(hhbc::Instruct::Opcode(hhbc::Opcode::Double(
        hhbc::FloatBits(num),
    )))
}

fn assemble_name<'arena, 'a>(
    alloc: &'arena Bump,
    token_iter: &mut Lexer<'a>,
) -> Result<hhbc::FunctionName<'arena>> {
    let name = token_iter.expect(Token::into_identifier)?;
    let fname = hhbc::FunctionName::new(Str::new_slice(alloc, name));
    Ok(fname)
}

#[derive(Parser, Debug)]
pub struct Opts {
    /// Output file. Creates it if necessary
    #[clap(short = 'o')]
    output_file: Option<PathBuf>,

    /// The input hhas file(s) to deserialize back to HackCUnit
    #[clap(flatten)]
    files: FileOpts,
}

type SyncWrite = Mutex<Box<dyn Write + Sync + Send>>;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Pos {
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Token<'a> {
    // See below in Lexer::from_slice for regex definitions
    Global(&'a [u8], Pos),
    Variable(&'a [u8], Pos),
    TripleStrLiteral(&'a [u8], Pos),
    Decl(&'a [u8], Pos),
    StrLiteral(&'a [u8], Pos),
    Variadic(Pos),
    Semicolon(Pos),
    Dash(Pos),
    OpenCurly(Pos),
    OpenBracket(Pos),
    OpenParen(Pos),
    CloseParen(Pos),
    CloseBracket(Pos),
    CloseCurly(Pos),
    Equal(Pos),
    Number(&'a [u8], Pos),
    Comma(Pos),
    Lt(Pos),
    Gt(Pos),
    Colon(Pos),
    Identifier(&'a [u8], Pos),
    Newline(Pos),
    Error(&'a [u8], Pos),
}

impl<'a> Token<'a> {
    fn as_bytes(&self) -> &'a [u8] {
        match self {
            Token::Global(u, _)
            | Token::Variable(u, _)
            | Token::TripleStrLiteral(u, _)
            | Token::Decl(u, _)
            | Token::StrLiteral(u, _)
            | Token::Number(u, _)
            | Token::Identifier(u, _)
            | Token::Error(u, _) => u,
            Token::Semicolon(_) => b";",
            Token::Dash(_) => b"-",
            Token::OpenCurly(_) => b"{",
            Token::OpenBracket(_) => b"[",
            Token::OpenParen(_) => b"(",
            Token::CloseParen(_) => b")",
            Token::CloseBracket(_) => b"]",
            Token::CloseCurly(_) => b"}",
            Token::Equal(_) => b"=",
            Token::Comma(_) => b",",
            Token::Lt(_) => b"<",
            Token::Gt(_) => b">",
            Token::Colon(_) => b":",
            Token::Variadic(_) => b"...",
            Token::Newline(_) => b"\n",
        }
    }

    /// Only str_literal and triple_str_literal can be parsed into a new tokenizer.
    /// To create a new tokenizer that still has accurate error reporting, we want to pass the line
    /// So `into_str_literal_and_line` and `into_triple_str_literal_and_line` return a Result of bytes rep and line # or bail
    fn into_triple_str_literal_and_line(self) -> Result<(&'a [u8], usize)> {
        match self {
            Token::TripleStrLiteral(vec_u8, pos) => Ok((vec_u8, pos.line)),
            _ => bail!("Expected a triple str literal, got: {}", self),
        }
    }

    #[allow(dead_code)]
    fn into_str_literal_and_line(self) -> Result<(&'a [u8], usize)> {
        match self {
            Token::StrLiteral(vec_u8, pos) => Ok((vec_u8, pos.line)),
            _ => bail!("Expected a str literal, got: {}", self),
        }
    }

    /// The "into" series of methods both check that [self] is the correct
    /// variant of Token and return a Result of [self]'s string rep
    #[allow(dead_code)]
    fn into_newline(self) -> Result<&'a [u8]> {
        match self {
            Token::Newline(_) => Ok(self.as_bytes()),
            _ => bail!("Expected a newline, got: {}", self),
        }
    }
    #[allow(dead_code)]
    fn into_global(self) -> Result<&'a [u8]> {
        match self {
            Token::Global(vec_u8, _) => Ok(vec_u8),
            _ => bail!("Expected a global, got: {}", self),
        }
    }

    #[allow(dead_code)]
    fn into_variable(self) -> Result<&'a [u8]> {
        match self {
            Token::Variable(vec_u8, _) => Ok(vec_u8),
            _ => bail!("Expected a variable, got: {}", self),
        }
    }

    #[allow(dead_code)]
    fn into_triple_str_literal(self) -> Result<&'a [u8]> {
        match self {
            Token::TripleStrLiteral(vec_u8, _) => Ok(vec_u8),
            _ => bail!("Expected a triple str literal, got: {}", self),
        }
    }

    fn into_decl(self) -> Result<&'a [u8]> {
        match self {
            Token::Decl(vec_u8, _) => Ok(vec_u8),
            _ => bail!("Expected a decl, got: {}", self),
        }
    }

    fn into_str_literal(self) -> Result<&'a [u8]> {
        match self {
            Token::StrLiteral(vec_u8, _) => Ok(vec_u8),
            _ => bail!("Expected a str literal, got: {}", self),
        }
    }

    fn into_number(self) -> Result<&'a [u8]> {
        match self {
            Token::Number(vec_u8, _) => Ok(vec_u8),
            _ => bail!("Expected a number, got: {}", self),
        }
    }

    fn into_identifier(self) -> Result<&'a [u8]> {
        match self {
            Token::Identifier(vec_u8, _) => Ok(vec_u8),
            _ => bail!("Expected an identifier, got: {}", self),
        }
    }

    #[allow(dead_code)]
    fn into_error(self) -> Result<&'a [u8]> {
        match self {
            Token::Error(vec_u8, _) => Ok(vec_u8),
            _ => bail!("Expected an error, got: {}", self),
        }
    }

    fn into_semicolon(self) -> Result<&'a [u8]> {
        match self {
            Token::Semicolon(_) => Ok(self.as_bytes()),
            _ => bail!("Expected a semicolon, got: {}", self),
        }
    }

    #[allow(dead_code)]
    fn into_dash(self) -> Result<&'a [u8]> {
        match self {
            Token::Dash(_) => Ok(self.as_bytes()),
            _ => bail!("Expected a dash, got: {}", self),
        }
    }

    fn into_open_curly(self) -> Result<&'a [u8]> {
        match self {
            Token::OpenCurly(_) => Ok(self.as_bytes()),
            _ => bail!("Expected an open curly, got: {}", self),
        }
    }

    #[allow(dead_code)]
    fn into_open_bracket(self) -> Result<&'a [u8]> {
        match self {
            Token::OpenBracket(_) => Ok(self.as_bytes()),
            _ => bail!("Expected an open bracket, got: {}", self),
        }
    }

    fn into_open_paren(self) -> Result<&'a [u8]> {
        match self {
            Token::OpenParen(_) => Ok(self.as_bytes()),
            _ => bail!("Expected an open paren, got: {}", self),
        }
    }

    fn into_close_paren(self) -> Result<&'a [u8]> {
        match self {
            Token::CloseParen(_) => Ok(self.as_bytes()),
            _ => bail!("Expected a close paren, got: {}", self),
        }
    }

    fn into_close_bracket(self) -> Result<&'a [u8]> {
        match self {
            Token::CloseBracket(_) => Ok(self.as_bytes()),
            _ => bail!("Expected a close bracket, got: {}", self),
        }
    }

    fn into_close_curly(self) -> Result<&'a [u8]> {
        match self {
            Token::CloseCurly(_) => Ok(self.as_bytes()),
            _ => bail!("Expected a close curly, got: {}", self),
        }
    }

    #[allow(dead_code)]
    fn into_equal(self) -> Result<&'a [u8]> {
        match self {
            Token::Equal(_) => Ok(self.as_bytes()),
            _ => bail!("Expected an equal, got: {}", self),
        }
    }

    fn into_comma(self) -> Result<&'a [u8]> {
        match self {
            Token::Comma(_) => Ok(self.as_bytes()),
            _ => bail!("Expected a comma, got: {}", self),
        }
    }

    fn into_lt(self) -> Result<&'a [u8]> {
        match self {
            Token::Lt(_) => Ok(self.as_bytes()),
            _ => bail!("Expected a lt (<), got: {}", self),
        }
    }

    fn into_gt(self) -> Result<&'a [u8]> {
        match self {
            Token::Gt(_) => Ok(self.as_bytes()),
            _ => bail!("Expected a gt (>), got: {}", self),
        }
    }

    fn into_colon(self) -> Result<&'a [u8]> {
        match self {
            Token::Colon(_) => Ok(self.as_bytes()),
            _ => bail!("Expected a colon, got: {}", self),
        }
    }

    #[allow(dead_code)]
    fn into_variadic(self) -> Result<&'a [u8]> {
        match self {
            Token::Variadic(_) => Ok(self.as_bytes()),
            _ => bail!("Expected a variadic, got: {}", self),
        }
    }

    //is
    #[allow(dead_code)]
    fn is_newline(&self) -> bool {
        matches!(self, Token::Newline(..))
    }
    #[allow(dead_code)]
    fn is_global(&self) -> bool {
        matches!(self, Token::Global(..))
    }

    #[allow(dead_code)]
    fn is_variable(&self) -> bool {
        matches!(self, Token::Variable(..))
    }

    #[allow(dead_code)]
    fn is_triple_str_literal(&self) -> bool {
        matches!(self, Token::TripleStrLiteral(..))
    }

    #[allow(dead_code)]
    fn is_decl(&self) -> bool {
        matches!(self, Token::Decl(..))
    }

    #[allow(dead_code)]
    fn is_str_literal(&self) -> bool {
        matches!(self, Token::StrLiteral(..))
    }

    #[allow(dead_code)]
    fn is_number(&self) -> bool {
        matches!(self, Token::Number(..))
    }

    #[allow(dead_code)]
    fn is_identifier(&self) -> bool {
        matches!(self, Token::Identifier(..))
    }

    #[allow(dead_code)]
    fn is_error(&self) -> bool {
        matches!(self, Token::Error(..))
    }

    #[allow(dead_code)]
    fn is_semicolon(&self) -> bool {
        matches!(self, Token::Semicolon(_))
    }

    #[allow(dead_code)]
    fn is_dash(&self) -> bool {
        matches!(self, Token::Dash(_))
    }

    #[allow(dead_code)]
    fn is_open_curly(&self) -> bool {
        matches!(self, Token::OpenCurly(_))
    }

    #[allow(dead_code)]
    fn is_open_bracket(&self) -> bool {
        matches!(self, Token::OpenBracket(_))
    }

    #[allow(dead_code)]
    fn is_open_paren(&self) -> bool {
        matches!(self, Token::OpenParen(_))
    }

    #[allow(dead_code)]
    fn is_close_paren(&self) -> bool {
        matches!(self, Token::CloseParen(_))
    }

    #[allow(dead_code)]
    fn is_close_bracket(&self) -> bool {
        matches!(self, Token::CloseBracket(_))
    }

    #[allow(dead_code)]
    fn is_close_curly(&self) -> bool {
        matches!(self, Token::CloseCurly(_))
    }

    #[allow(dead_code)]
    fn is_equal(&self) -> bool {
        matches!(self, Token::Equal(_))
    }

    #[allow(dead_code)]
    fn is_comma(&self) -> bool {
        matches!(self, Token::Comma(_))
    }

    #[allow(dead_code)]
    fn is_lt(&self) -> bool {
        matches!(self, Token::Lt(_))
    }

    #[allow(dead_code)]
    fn is_gt(&self) -> bool {
        matches!(self, Token::Gt(_))
    }

    #[allow(dead_code)]
    fn is_colon(&self) -> bool {
        matches!(self, Token::Colon(_))
    }

    #[allow(dead_code)]
    fn is_variadic(&self) -> bool {
        matches!(self, Token::Variadic(_))
    }
}

impl<'a> fmt::Display for Token<'a> {
    /// Purpose of this fmt: so that vec of u8 (internal str representation of each token) is printed as a string rather than bytes
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = std::str::from_utf8(self.as_bytes()).map_err(|_| fmt::Error)?;
        match self {
            Token::Global(_, pos) => write!(f, "Global(\"{text}\", {pos:?})"),
            Token::Variable(_, pos) => write!(f, "Variable(\"{text}\", {pos:?})"),
            Token::TripleStrLiteral(_, pos) => write!(f, "TripleStrLiteral(\"{text}\", {pos:?})"),
            Token::Decl(_, pos) => write!(f, "Decl(\"{text}\", {pos:?})"),
            Token::StrLiteral(_, pos) => write!(f, "StrLiteral(\"{text}\", {pos:?})"),
            Token::Number(_, pos) => write!(f, "Number(\"{text}\", {pos:?})"),
            Token::Identifier(_, pos) => write!(f, "Identifier(\"{text}\", {pos:?})"),
            Token::Error(_, pos) => write!(f, "Error(\"{text}\", {pos:?})"),
            Token::Semicolon(pos) => write!(f, "Semicolon(\"{text}\", {pos:?})"),
            Token::Dash(pos) => write!(f, "Dash(\"{text}\", {pos:?})"),
            Token::OpenCurly(pos) => write!(f, "OpenCurly(\"{text}\", {pos:?})"),
            Token::OpenBracket(pos) => write!(f, "OpenBracket(\"{text}\", {pos:?})"),
            Token::OpenParen(pos) => write!(f, "OpenParen(\"{text}\", {pos:?})"),
            Token::CloseParen(pos) => write!(f, "CloseParen(\"{text}\", {pos:?})"),
            Token::CloseBracket(pos) => write!(f, "CloseBracket(\"{text}\", {pos:?})"),
            Token::CloseCurly(pos) => write!(f, "CloseCurly(\"{text}\", {pos:?})"),
            Token::Equal(pos) => write!(f, "Equal(\"{text}\", {pos:?})"),
            Token::Comma(pos) => write!(f, "Comma(\"{text}\", {pos:?})"),
            Token::Lt(pos) => write!(f, "Lt(\"{text}\", {pos:?})"),
            Token::Gt(pos) => write!(f, "Gt(\"{text}\", {pos:?})"),
            Token::Colon(pos) => write!(f, "Colon(\"{text}\", {pos:?})"),
            Token::Variadic(pos) => write!(f, "Variadic(\"{text}\", {pos:?})"),
            Token::Newline(pos) => write!(f, "Newline(\"{text}\", {pos:?})"),
        }
    }
}

// We initially planned on using Logos, a crate for tokenizing really fast.
// We chose not to use Logos because it doesn't support all regexes -- for instance, it can't
// tokenize based on the regex "\"\"\".*\"\"\"". Here's the git issue:
// https://github.com/maciejhirsz/logos/issues/246
pub struct Lexer<'a> {
    pending: Option<Token<'a>>, // Cached next (non-newline)
    tokens: VecDeque<Token<'a>>,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    /// Returns the next token, never returning a newline and instead advancing the lexer past them.
    fn next(&mut self) -> Option<Self::Item> {
        self.fill(); // If `pending` already has a token this does nothing
        self.pending.take()
    }
}

impl<'a> Lexer<'a> {
    /// Counts tokens in this `Lexer`, including newlines. Because of this it only makes sense to call
    /// `size` on `Lexer`s spawned from `fetch_until_newline`
    fn _size(&self) -> usize {
        if self.pending.is_some() {
            1 + self.tokens.len()
        } else {
            self.tokens.len()
        }
    }

    /// If `pending` is none, fills with the next non-newline token, or None of one does not exist.
    fn fill(&mut self) {
        if self.pending.is_none() {
            self.pending = self.get_next_no_newlines();
        }
    }

    /// Returns the next non-newline token (or None), advancing the lexer past any leading newlines.
    fn get_next_no_newlines(&mut self) -> Option<Token<'a>> {
        while let Some(t) = self.tokens.pop_front() {
            if !t.is_newline() {
                return Some(t);
            }
        }
        None
    }

    pub fn is_empty(&mut self) -> bool {
        self.fill();
        if self.pending.is_none() {
            debug_assert!(self.tokens.is_empty()); // VD should never have \n \n \n and empty pending
            true
        } else {
            false
        }
    }

    /// Advances the lexer passed its first non-leading newline, returning a mini-lexer of the tokens up until that newline.
    pub fn fetch_until_newline(&mut self) -> Option<Lexer<'a>> {
        self.fill();
        if let Some(t) = self.pending {
            let mut first_toks = VecDeque::new();
            // Add what was in `pending`
            first_toks.push_back(t);
            // While we haven't reached the new line
            while let Some(to_push) = self.tokens.pop_front() {
                if to_push.is_newline() {
                    break;
                }
                // Add token
                first_toks.push_back(to_push);
            }
            self.pending = None;
            Some(Lexer {
                tokens: first_toks,
                pending: None,
            })
        } else {
            None
        }
    }

    /// Similarly to `Lexer::next`, will not return a peek to a newline. Instead
    /// this `peek` returns a view to the first token that's not a newline, and modifies the lexer in that it strips
    /// it of leading newlines.
    pub fn peek(&mut self) -> Option<&Token<'a>> {
        self.fill();
        self.pending.as_ref()
    }

    /// Returns f applied to the top of the iterator, but does not advance iterator (unlike next_if)
    fn peek_if<F>(&mut self, f: F) -> bool
    where
        F: Fn(&Token<'a>) -> bool,
    {
        self.peek().map_or(false, f)
    }

    /// Applies f to top of iterator, and if true advances iterator and returns true else doesn't advance and returns false
    fn next_if<F>(&mut self, f: F) -> bool
    where
        F: Fn(&Token<'a>) -> bool,
    {
        let tr = self.peek_if(f);
        if tr {
            self.next();
        }
        tr
    }

    /// Applies f to top of iterator. If true, compares inner rep to passed &str and returns result. Else just false
    fn peek_if_str<F>(&mut self, f: F, s: &str) -> bool
    where
        F: Fn(&Token<'a>) -> bool,
    {
        self.peek_if(|t| f(t) && t.as_bytes() == s.as_bytes())
    }

    /// Applies f to top of iterator. If true, compares inner representation to passed &str and if true consumes. Else doesn't modify iterator and returns false.
    fn next_if_str<F>(&mut self, f: F, s: &str) -> bool
    where
        F: Fn(&Token<'a>) -> bool,
    {
        let tr = self.peek_if_str(f, s);
        if tr {
            self.next();
        }
        tr
    }

    /// Applies f to the top of token_iter. In the most likely use, f bails if token_iter is not the
    /// expected token (expected token specified by f)
    fn expect<T, F>(&mut self, f: F) -> Result<T>
    where
        F: FnOnce(Token<'a>) -> Result<T>,
    {
        self.next()
            .map_or_else(|| bail!("End of token stream sooner than expected"), f)
    }

    /// Like `expect` in that bails if incorrect token passed, but dissimilarly does not return the `into` of the passed token
    fn expect_is_str<F>(&mut self, f: F, s: &str) -> Result<()>
    where
        F: FnOnce(Token<'a>) -> Result<&[u8]>,
    {
        self.next().map_or_else(
            || bail!("End of token stream sooner than expected"),
            |t| {
                if f(t)? != s.as_bytes() {
                    bail!("Expected {} got {}", s, t)
                } else {
                    Ok(())
                }
            },
        )
    }

    /// Similar to `expect` but instead of returning a Result that usually contains a slice of u8,
    /// applies f to the `from_utf8 str` of the top token, bailing if the top token is not a number.
    fn expect_and_get_number<T: FromStr>(&mut self) -> Result<T> {
        let num = self.expect(Token::into_number)?;
        FromStr::from_str(std::str::from_utf8(num)?).map_err(|_| {
            anyhow!("Number-looking token in tokenizer that cannot be parsed into number")
        }) // This std::str::from_utf8 will never bail; if it should bail, the above `expect` bails first.
    }

    /// Bails if lexer is not empty, message contains the next token
    fn expect_end(&mut self) -> Result<()> {
        if !self.is_empty() {
            bail!(
                "Expected end of token stream, see: {}",
                self.next().unwrap()
            )
        }
        Ok(())
    }

    pub fn from_slice(s: &'a [u8], start_line: usize) -> Self {
        // First create the regex that matches any token. Done this way for readability
        let v = [
            r#"""".*""""#,                                          // Triple str literal
            "#.*",                                                  // Comment
            r"(?-u)[\.@][_a-zA-Z\x80-\xff][_/a-zA-Z0-9\x80-\xff]*", // Decl, global. (?-u) turns off utf8 check
            r"(?-u)\$[_a-zA-Z0-9\x80-\xff][_/a-zA-Z0-9\x80-\xff]*", // Var. See /home/almathaler/fbsource/fbcode/hphp/test/quick/reified-and-variadic.php's assembly for a var w/ a digit at front
            r#""((\\.)|[^\\"])*""#,                                 // Str literal
            r"[-+]?[0-9]+\.?[0-9]*",                                // Number
            r"(?-u)[_/a-zA-Z\x80-\xff][_/\\a-zA-Z0-9\x80-\xff]*",   // Identifier
            ";",
            "-",
            "=",
            r"\{",
            r"\[",
            r"\(",
            r"\)",
            r"\]",
            r"\}",
            ",",
            "<",
            ">",
            ":",
            r"\.\.\.", // Variadic
            "\n",
            r"[ \t\r\f]+",
        ];
        let big_regex = format!("^(({}))", v.join(")|("));
        let big_regex = Regex::new(&big_regex).unwrap();
        let mut cur_pos = Pos {
            line: start_line, // When we spawn a new lexer it doesn't start at line 1
            col: 1,
        };
        let mut tokens = VecDeque::new();
        let mut source = s;
        while !source.is_empty() {
            source = build_tokens_helper(source, &mut cur_pos, &mut tokens, &big_regex);
        }
        Lexer {
            pending: None,
            tokens,
        }
    }
}

fn build_tokens_helper<'a>(
    s: &'a [u8],
    cur_pos: &mut Pos,
    tokens: &mut VecDeque<Token<'a>>,
    big_regex: &Regex,
) -> &'a [u8] {
    if let Some(mat) = big_regex.find(s) {
        let mut chars = s.iter(); // Implicit assumption: matched to the start (^), so we iter from the start
        debug_assert!(mat.start() == 0);
        match chars.next().unwrap() {
            // Get first character
            // Note these don't match what prints out on a printer, but not sure how to generalize
            b'\x0C' => {
                // form feed
                cur_pos.col += 1;
                &s[mat.end()..]
            }
            b'\r' => {
                cur_pos.col = 1;
                &s[mat.end()..]
            }
            b'\t' => {
                cur_pos.col += 1; // Column not currently used in error reporting, so keep this as 1
                &s[mat.end()..]
            }
            b'#' => {
                &s[mat.end()..] // Don't advance the line; the newline at the end of the comment will advance the line
            }
            b' ' => {
                cur_pos.col += 1;
                &s[mat.end()..]
            } // Don't add whitespace as tokens, just increase line and col
            o => {
                let end = mat.end();
                let tok = match o {
                    b'\n' => {
                        let old_pos = Pos { ..*cur_pos };
                        cur_pos.line += 1;
                        cur_pos.col = 1;
                        Token::Newline(old_pos)
                    }
                    b'@' => Token::Global(&s[..end], *cur_pos), // Global
                    b'$' => Token::Variable(&s[..end], *cur_pos), // Var
                    b'.' => {
                        if *(chars.next().unwrap()) == b'.' && *(chars.next().unwrap()) == b'.' {
                            // Variadic
                            Token::Variadic(*cur_pos)
                        } else {
                            Token::Decl(&s[..end], *cur_pos) // Decl
                        }
                    }
                    b';' => Token::Semicolon(*cur_pos), // Semicolon
                    b'{' => Token::OpenCurly(*cur_pos), // Opencurly
                    b'[' => Token::OpenBracket(*cur_pos),
                    b'(' => Token::OpenParen(*cur_pos),
                    b')' => Token::CloseParen(*cur_pos),
                    b']' => Token::CloseBracket(*cur_pos),
                    b'}' => Token::CloseCurly(*cur_pos),
                    b',' => Token::Comma(*cur_pos),
                    b'<' => Token::Lt(*cur_pos),    //<
                    b'>' => Token::Gt(*cur_pos),    //>
                    b'=' => Token::Equal(*cur_pos), //=
                    b'-' => {
                        if chars.next().unwrap().is_ascii_digit() {
                            // Negative number
                            Token::Number(&s[..end], *cur_pos)
                        } else {
                            Token::Dash(*cur_pos)
                        }
                    }
                    b':' => Token::Colon(*cur_pos),
                    b'"' => {
                        if *(chars.next().unwrap()) == b'"' && *(chars.next().unwrap()) == b'"' {
                            // Triple string literal
                            Token::TripleStrLiteral(&s[..end], *cur_pos)
                        } else {
                            // Single string literal
                            Token::StrLiteral(&s[..end], *cur_pos)
                        }
                    }
                    dig_or_id => {
                        if dig_or_id.is_ascii_digit()
                            || (*dig_or_id as char == '+'
                                && (chars.next().unwrap()).is_ascii_digit())
                        // Positive numbers denoted with +
                        {
                            Token::Number(&s[..end], *cur_pos)
                        } else {
                            Token::Identifier(&s[..end], *cur_pos)
                        }
                    }
                };
                tokens.push_back(tok);
                cur_pos.col += end - mat.start(); // Advance col by length of token
                &s[end..]
            }
        }
    } else {
        // Couldn't tokenize the string, so add the rest of it as an error
        tokens.push_back(Token::Error(
            s,
            Pos {
                line: cur_pos.line,
                col: cur_pos.col,
            },
        ));
        // Done advancing col and line cuz at end
        &[]
    }
}

/// Tokenizes input string using a Lexer. Prints all tokens in the Lexer
fn _print_tokens<'a>(s: &'a [u8]) {
    let lex: Lexer<'a> = Lexer::from_slice(s, 1);
    for tok in lex {
        println!("{}", tok);
    }
}

fn _print_from_lexer<'a>(l: Lexer<'a>) {
    for tok in l {
        println!("{}", tok);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn str_into_test() -> Result<()> {
        // Want to test that only tokens surrounded by "" are str_literals
        // Want to confirm the assumption that after any token_iter.expect(Token::into_str_literal) call, you can safely remove the first and last element in slice
        let s = r#"abc "abc" """abc""""#;
        let s = s.as_bytes();
        let mut lex = Lexer::from_slice(s, 1);
        assert!(lex.next_if(Token::is_identifier));
        let sl = lex.expect(Token::into_str_literal)?;
        assert!(sl[0] == b'"' && sl[sl.len() - 1] == b'"');
        let tsl = lex.expect(Token::into_triple_str_literal)?;
        assert!(
            tsl[0..3] == [b'"', b'"', b'"'] && tsl[tsl.len() - 3..tsl.len()] == [b'"', b'"', b'"']
        );
        Ok(())
    }
    #[test]
    fn just_nl_is_empty() {
        let s = "\n \n \n";
        let s = s.as_bytes();
        let mut lex = Lexer::from_slice(s, 1);
        assert!(lex.fetch_until_newline().is_none());
        assert!(lex.is_empty());
    }
    #[test]
    fn splits_mult_newlines_go_away() {
        // Point of this test: want to make sure that 3 mini-lexers are spawned (multiple new lines don't do anything)
        let s = "\n \n a \n \n \n b \n \n c \n";
        let s = s.as_bytes();
        let mut lex = Lexer::from_slice(s, 1);
        let vc_of_lexers = vec![
            lex.fetch_until_newline(),
            lex.fetch_until_newline(),
            lex.fetch_until_newline(),
        ];
        assert!(lex.is_empty());
        assert!(lex.next().is_none());
        assert_eq!(vc_of_lexers.len(), 3);
    }
    #[test]
    fn no_trailing_newlines() {
        let s = "a \n \n \n";
        let s = s.as_bytes();
        let mut lex = Lexer::from_slice(s, 1);
        assert!(lex.next().is_some());
        assert!(lex.is_empty());
    }
    #[test]
    fn splitting_multiple_lines() {
        let s = ".try { \n .srloc 3:7, 3:22 \n String \"I'm in the try\n\" \n Print \n PopC \n } .catch { \n Dup \n L1: \n Throw \n }";
        let s = s.as_bytes();
        let mut lex = Lexer::from_slice(s, 1);
        let mut vc_of_lexers = Vec::new();
        while !lex.is_empty() {
            vc_of_lexers.push(lex.fetch_until_newline());
        }
        assert_eq!(vc_of_lexers.len(), 10)
    }
    #[test]
    fn peek_next_on_newlines() {
        let s = "\n\na\n\n";
        let mut lex = Lexer::from_slice(s.as_bytes(), 1);
        assert!(lex.peek().is_some());
        assert!(lex.next().is_some());
        assert!(lex.fetch_until_newline().is_none()); // Have consumed the a here -- "\n\n" was left and that's been consumed.
    }
    #[test]
    #[should_panic]
    fn no_top_level_shouldnt_parse() {
        // Is there a better way, maybe to verify the string in the bail?
        let s = ".srloc 3:7,3:22";
        let s = s.as_bytes();
        let alloc = Bump::default();
        assert!(matches!(assemble_from_bytes(&alloc, s), Ok(_)))
    }
    #[test]
    #[should_panic]
    fn no_fpath_semicolon_shouldnt_parse() {
        let s = r#".filepath "aaaa""#;
        let s = s.as_bytes();
        let alloc = Bump::default();
        assert!(matches!(assemble_from_bytes(&alloc, s), Ok(_)))
    }
    #[test]
    #[should_panic]
    fn fpath_wo_file_shouldnt_parse() {
        let s = r#".filepath aaa"#;
        let s = s.as_bytes();
        let alloc = Bump::default();
        assert!(matches!(assemble_from_bytes(&alloc, s), Ok(_)))
    }
    #[test]
    fn difficult_strings() {
        let s = r#""\"0\""
        "12345\\:2\\"
        "class_meth() expects a literal class name or ::class constant, followed by a constant string that refers to a static method on that class";
        "#;
        let s = s.as_bytes();
        let l: Lexer<'_> = Lexer::from_slice(s, 1);
        let mut l = l.into_iter();
        // Expecting 3 string tokens
        let _st1 = l.next().unwrap();
        let _by1 = str::as_bytes(r#""\"0\"""#);
        assert!(matches!(_st1, Token::StrLiteral(_by1, _)));
        let _st2 = l.next().unwrap();
        let _by2 = str::as_bytes(r#""12345\\:2\\""#);
        assert!(matches!(_st1, Token::StrLiteral(_by2, _)));
        let _st3 = l.next().unwrap();
        let _by3 = str::as_bytes(
            r#""class_meth() expects a literal class name or ::class constant, followed by a constant string that refers to a static method on that class""#,
        );
        assert!(matches!(_st1, Token::StrLiteral(_by3, _)));
    }
    #[test]
    fn odd_unicode_test() {
        let s: &[u8] = b".\xA9\xEF\xB8\x8E $0\xC5\xA3\xB1\xC3 \xE2\x98\xBA\xE2\x98\xBA\xE2\x98\xBA @\xE2\x99\xA1\xE2\x99\xA4$";
        let l: Lexer<'_> = Lexer::from_slice(s, 1);
        // We are expecting an decl, a var, an identifier a global, and an error on the last empty variable
        let mut l = l.into_iter();
        let decl = l.next().unwrap();
        assert!(matches!(decl, Token::Decl(..)));
        let var = l.next().unwrap();
        assert!(matches!(var, Token::Variable(..)));
        let iden = l.next().unwrap();
        assert!(matches!(iden, Token::Identifier(..)));
        let glob = l.next().unwrap();
        assert!(matches!(glob, Token::Global(..)));
        let err = l.next().unwrap();
        assert!(matches!(err, Token::Error(..)))
    }

    #[test]
    fn every_token_test() {
        let s = r#"@_global $0Var """tripleStrLiteral:)""" #hashtagComment
        .Decl "str!Literal" ...
        ;-{[( )]} =98 -98 +101. 43.2 , < > : _/identifier/ /filepath ."#;
        // Expect glob var tsl decl strlit semicolon dash open_curly open_brack open_paren close_paren close_bracket
        // close_curly equal number number number number , < > : identifier identifier ERROR on the last .
        let s = s.as_bytes();
        let l: Lexer<'_> = Lexer::from_slice(s, 1);
        let mut l = l.into_iter();
        let glob = l.next().unwrap();
        assert!(
            matches!(glob, Token::Global(..)),
            "failed to match {}",
            glob
        );
        let var = l.next().unwrap();
        assert!(
            matches!(var, Token::Variable(..)),
            "failed to match {}",
            var
        );
        let tsl = l.next().unwrap();
        assert!(
            matches!(tsl, Token::TripleStrLiteral(..)),
            "failed to match {}",
            tsl
        );
        let decl = l.next().unwrap();
        assert!(matches!(decl, Token::Decl(..)), "failed to match {}", decl);
        let strlit = l.next().unwrap();
        assert!(
            matches!(strlit, Token::StrLiteral(..)),
            "failed to match {}",
            strlit
        );
        let variadic = l.next().unwrap();
        assert!(
            matches!(variadic, Token::Variadic(..)),
            "failed to match {}",
            variadic
        );
        let semicolon = l.next().unwrap();
        assert!(
            matches!(semicolon, Token::Semicolon(..)),
            "failed to match {}",
            semicolon
        );
        let dash = l.next().unwrap();
        assert!(matches!(dash, Token::Dash(..)), "failed to match {}", dash);
        let oc = l.next().unwrap();
        assert!(matches!(oc, Token::OpenCurly(..)), "failed to match {}", oc);
        let ob = l.next().unwrap();
        assert!(
            matches!(ob, Token::OpenBracket(..)),
            "failed to match {}",
            ob
        );
        let op = l.next().unwrap();
        assert!(matches!(op, Token::OpenParen(..)), "failed to match {}", op);
        let cp = l.next().unwrap();
        assert!(
            matches!(cp, Token::CloseParen(..)),
            "failed to match {}",
            cp
        );
        let cb = l.next().unwrap();
        assert!(
            matches!(cb, Token::CloseBracket(..)),
            "failed to match {}",
            cb
        );
        let cc = l.next().unwrap();
        assert!(
            matches!(cc, Token::CloseCurly(..)),
            "failed to match {}",
            cc
        );
        let eq = l.next().unwrap();
        assert!(matches!(eq, Token::Equal(..)), "failed to match {}", eq);
        let num = l.next().unwrap();
        assert!(matches!(num, Token::Number(..)), "failed to match {}", num);
        let num = l.next().unwrap();
        assert!(matches!(num, Token::Number(..)), "failed to match {}", num);
        let num = l.next().unwrap();
        assert!(matches!(num, Token::Number(..)), "failed to match {}", num);
        let num = l.next().unwrap();
        assert!(matches!(num, Token::Number(..)), "failed to match {}", num);
        let comma = l.next().unwrap();
        assert!(
            matches!(comma, Token::Comma(..)),
            "failed to match {}",
            comma
        );
        let lt = l.next().unwrap();
        assert!(matches!(lt, Token::Lt(..)), "failed to match {}", lt);
        let gt = l.next().unwrap();
        assert!(matches!(gt, Token::Gt(..)), "failed to match {}", gt);
        let colon = l.next().unwrap();
        assert!(
            matches!(colon, Token::Colon(..)),
            "failed to match {}",
            colon
        );
        let iden = l.next().unwrap();
        assert!(
            matches!(iden, Token::Identifier(..)),
            "failed to match {}",
            iden
        );
        let iden = l.next().unwrap();
        assert!(
            matches!(iden, Token::Identifier(..)),
            "failed to match {}",
            iden
        );
        let err = l.next().unwrap();
        assert!(matches!(err, Token::Error(..)), "failed to match {}", err);
    }
}
