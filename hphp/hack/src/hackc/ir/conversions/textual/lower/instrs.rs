// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.

use std::sync::Arc;

use ir::func_builder::TransformInstr;
use ir::func_builder::TransformState;
use ir::instr::CallDetail;
use ir::instr::CmpOp;
use ir::instr::HasLoc;
use ir::instr::HasOperands;
use ir::instr::Hhbc;
use ir::instr::IteratorArgs;
use ir::instr::Predicate;
use ir::instr::Special;
use ir::instr::Terminator;
use ir::instr::Textual;
use ir::BareThisOp;
use ir::Call;
use ir::Constant;
use ir::FCallArgsFlags;
use ir::Func;
use ir::FuncBuilder;
use ir::FuncBuilderEx as _;
use ir::Instr;
use ir::InstrId;
use ir::IsTypeOp;
use ir::LocId;
use ir::LocalId;
use ir::MethodId;
use ir::ObjMethodOp;
use ir::SpecialClsRef;
use ir::TypeStructResolveOp;
use ir::UnitBytesId;
use ir::ValueId;
use itertools::Itertools;

use super::func_builder::FuncBuilderEx as _;
use crate::class::IsStatic;
use crate::func::MethodInfo;
use crate::hack;

/// Lower individual Instrs in the Func to simpler forms.
pub(crate) fn lower_instrs(
    builder: &mut FuncBuilder<'_>,
    method_info: Option<Arc<MethodInfo<'_>>>,
) {
    let mut lowerer = LowerInstrs {
        changed: false,
        method_info,
    };

    let mut bid = Func::ENTRY_BID;
    while bid.0 < builder.func.blocks.len() as u32 {
        lowerer.changed = true;
        while lowerer.changed {
            // The lowered instructions may have emitted stuff that needs to be
            // lowered again. This could be optimized by only setting `changed`
            // when we think we need to loop.
            lowerer.changed = false;
            builder.rewrite_block(bid, &mut lowerer);
        }
        bid.0 += 1;
    }
}

struct LowerInstrs<'a> {
    changed: bool,
    method_info: Option<Arc<MethodInfo<'a>>>,
}

impl LowerInstrs<'_> {
    fn handle_hhbc_with_builtin(&self, hhbc: &Hhbc) -> Option<hack::Hhbc> {
        let builtin = match hhbc {
            Hhbc::Add(..) => hack::Hhbc::Add,
            Hhbc::AddElemC(..) => hack::Hhbc::AddElemC,
            Hhbc::AddNewElemC(..) => hack::Hhbc::AddNewElemC,
            Hhbc::CastVec(..) => hack::Hhbc::CastVec,
            Hhbc::ClassGetC(..) => hack::Hhbc::ClassGetC,
            Hhbc::ClassHasReifiedGenerics(..) => hack::Hhbc::ClassHasReifiedGenerics,
            Hhbc::CheckClsRGSoft(..) => hack::Hhbc::CheckClsRGSoft,
            Hhbc::CmpOp(_, CmpOp::Eq, _) => hack::Hhbc::CmpEq,
            Hhbc::CmpOp(_, CmpOp::Gt, _) => hack::Hhbc::CmpGt,
            Hhbc::CmpOp(_, CmpOp::Gte, _) => hack::Hhbc::CmpGte,
            Hhbc::CmpOp(_, CmpOp::Lt, _) => hack::Hhbc::CmpLt,
            Hhbc::CmpOp(_, CmpOp::Lte, _) => hack::Hhbc::CmpLte,
            Hhbc::CmpOp(_, CmpOp::NSame, _) => hack::Hhbc::CmpNSame,
            Hhbc::CmpOp(_, CmpOp::Neq, _) => hack::Hhbc::CmpNeq,
            Hhbc::CmpOp(_, CmpOp::Same, _) => hack::Hhbc::CmpSame,
            Hhbc::CombineAndResolveTypeStruct(..) => hack::Hhbc::CombineAndResolveTypeStruct,
            Hhbc::Concat(..) => hack::Hhbc::Concat,
            Hhbc::ConcatN(..) => hack::Hhbc::ConcatN,
            Hhbc::Div(..) => hack::Hhbc::Div,
            Hhbc::GetClsRGProp(..) => hack::Hhbc::GetClsRGProp,
            Hhbc::HasReifiedParent(..) => hack::Hhbc::HasReifiedParent,
            Hhbc::IsLateBoundCls(..) => hack::Hhbc::IsLateBoundCls,
            Hhbc::IsTypeC(_, IsTypeOp::ArrLike, _) => hack::Hhbc::IsTypeArrLike,
            Hhbc::IsTypeC(_, IsTypeOp::Bool, _) => hack::Hhbc::IsTypeBool,
            Hhbc::IsTypeC(_, IsTypeOp::Class, _) => hack::Hhbc::IsTypeClass,
            Hhbc::IsTypeC(_, IsTypeOp::ClsMeth, _) => hack::Hhbc::IsTypeClsMeth,
            Hhbc::IsTypeC(_, IsTypeOp::Dbl, _) => hack::Hhbc::IsTypeDbl,
            Hhbc::IsTypeC(_, IsTypeOp::Dict, _) => hack::Hhbc::IsTypeDict,
            Hhbc::IsTypeC(_, IsTypeOp::Func, _) => hack::Hhbc::IsTypeFunc,
            Hhbc::IsTypeC(_, IsTypeOp::Int, _) => hack::Hhbc::IsTypeInt,
            Hhbc::IsTypeC(_, IsTypeOp::Keyset, _) => hack::Hhbc::IsTypeKeyset,
            Hhbc::IsTypeC(_, IsTypeOp::LegacyArrLike, _) => hack::Hhbc::IsTypeLegacyArrLike,
            Hhbc::IsTypeC(_, IsTypeOp::Null, _) => hack::Hhbc::IsTypeNull,
            Hhbc::IsTypeC(_, IsTypeOp::Obj, _) => hack::Hhbc::IsTypeObj,
            Hhbc::IsTypeC(_, IsTypeOp::Res, _) => hack::Hhbc::IsTypeRes,
            Hhbc::IsTypeC(_, IsTypeOp::Scalar, _) => hack::Hhbc::IsTypeScalar,
            Hhbc::IsTypeC(_, IsTypeOp::Str, _) => hack::Hhbc::IsTypeStr,
            Hhbc::IsTypeC(_, IsTypeOp::Vec, _) => hack::Hhbc::IsTypeVec,
            Hhbc::Modulo(..) => hack::Hhbc::Modulo,
            Hhbc::Mul(..) => hack::Hhbc::Mul,
            Hhbc::NewDictArray(..) => hack::Hhbc::NewDictArray,
            Hhbc::NewKeysetArray(..) => hack::Hhbc::NewKeysetArray,
            Hhbc::NewVec(..) => hack::Hhbc::NewVec,
            Hhbc::Not(..) => hack::Hhbc::Not,
            Hhbc::Print(..) => hack::Hhbc::Print,
            Hhbc::RecordReifiedGeneric(..) => hack::Hhbc::RecordReifiedGeneric,
            Hhbc::Sub(..) => hack::Hhbc::Sub,
            _ => return None,
        };
        Some(builtin)
    }

    fn handle_terminator_with_builtin(&self, term: &Terminator) -> Option<hack::Hhbc> {
        let builtin = match term {
            Terminator::Exit(..) => hack::Hhbc::Exit,
            Terminator::Throw(..) => hack::Hhbc::Throw,
            Terminator::ThrowAsTypeStructException(..) => hack::Hhbc::ThrowAsTypeStructException,
            _ => return None,
        };
        Some(builtin)
    }

    fn handle_with_builtin(&self, builder: &mut FuncBuilder<'_>, instr: &Instr) -> Option<Instr> {
        match instr {
            Instr::Hhbc(hhbc) => {
                let hhbc = self.handle_hhbc_with_builtin(hhbc)?;
                Some(builder.hhbc_builtin(hhbc, instr.operands(), instr.loc_id()))
            }
            Instr::Terminator(term) => {
                let hhbc = self.handle_terminator_with_builtin(term)?;
                builder.emit_hhbc_builtin(hhbc, instr.operands(), instr.loc_id());
                Some(Instr::unreachable())
            }
            _ => None,
        }
    }

    fn iter_init(
        &self,
        builder: &mut FuncBuilder<'_>,
        args: IteratorArgs,
        container: ValueId,
    ) -> Instr {
        // iterator ^0 init from %0 jmp to b2 else b1 with $index
        // ->
        // %n = hack::iter_init(&iter0, /* key */ null, &$index, %0)
        // if %n jmp b1 else b2

        let loc = args.loc;
        let iter_lid = iter_var_name(args.iter_id, &builder.strings);

        let iter_var = builder.emit(Textual::deref(iter_lid));

        let value_var = builder.emit(Textual::deref(args.value_lid()));

        let key_var = if let Some(key_lid) = args.key_lid() {
            builder.emit(Textual::deref(key_lid))
        } else {
            builder.emit_constant(Constant::Null)
        };

        let pred = builder.emit_hhbc_builtin(
            hack::Hhbc::IterInit,
            &[iter_var, key_var, value_var, container],
            loc,
        );

        Instr::jmp_op(
            pred,
            Predicate::NonZero,
            args.next_bid(),
            args.done_bid(),
            loc,
        )
    }

    fn iter_next(&self, builder: &mut FuncBuilder<'_>, args: IteratorArgs) -> Instr {
        // iterator ^0 next jmp to b2 else b1 with $index
        // ->
        // %n = hack::iter_next(&iter0, /* key */ null, &$index)
        // if %n jmp b1 else b2

        let loc = args.loc;
        let iter_lid = iter_var_name(args.iter_id, &builder.strings);

        let value_var = builder.emit(Textual::deref(args.value_lid()));

        let key_var = if let Some(key_lid) = args.key_lid() {
            builder.emit(Textual::deref(key_lid))
        } else {
            builder.emit_constant(Constant::Null)
        };

        let iter_var = builder.emit(Instr::Hhbc(Hhbc::CGetL(iter_lid, loc)));
        let pred =
            builder.emit_hhbc_builtin(hack::Hhbc::IterNext, &[iter_var, key_var, value_var], loc);

        Instr::jmp_op(
            pred,
            Predicate::NonZero,
            args.next_bid(),
            args.done_bid(),
            loc,
        )
    }

    fn verify_out_type(
        &self,
        builder: &mut FuncBuilder<'_>,
        obj: ValueId,
        lid: LocalId,
        loc: LocId,
    ) -> Instr {
        let param = builder
            .func
            .get_param_by_lid(lid)
            .expect("Unknown parameter in verify_out_type()");
        let param_type = param.ty.enforced.clone();
        let pred = builder.emit_is(obj, &param_type, loc);
        builder.emit_hack_builtin(hack::Builtin::VerifyTypePred, &[obj, pred], loc);
        Instr::copy(obj)
    }

    fn verify_param_type_ts(
        &self,
        builder: &mut FuncBuilder<'_>,
        lid: LocalId,
        ts: ValueId,
        loc: LocId,
    ) -> Instr {
        let obj = builder.emit(Instr::Hhbc(Hhbc::CGetL(lid, loc)));
        let builtin = hack::Hhbc::VerifyParamTypeTS;
        builder.emit_hhbc_builtin(builtin, &[obj, ts], loc);
        Instr::tombstone()
    }

    fn verify_ret_type_c(&self, builder: &mut FuncBuilder<'_>, obj: ValueId, loc: LocId) -> Instr {
        let return_type = builder.func.return_type.enforced.clone();
        let pred = builder.emit_is(obj, &return_type, loc);
        builder.emit_hack_builtin(hack::Builtin::VerifyTypePred, &[obj, pred], loc);
        Instr::copy(obj)
    }

    fn verify_ret_type_ts(
        &self,
        builder: &mut FuncBuilder<'_>,
        obj: ValueId,
        ts: ValueId,
        loc: LocId,
    ) -> Instr {
        let builtin = hack::Hhbc::VerifyParamTypeTS;
        builder.emit_hhbc_builtin(builtin, &[obj, ts], loc);
        Instr::copy(obj)
    }

    fn emit_special_cls_ref(
        &mut self,
        builder: &mut FuncBuilder<'_>,
        clsref: SpecialClsRef,
        loc: LocId,
    ) -> ValueId {
        match clsref {
            SpecialClsRef::SelfCls => builder.emit(Instr::Hhbc(Hhbc::SelfCls(loc))),
            SpecialClsRef::LateBoundCls => builder.emit(Instr::Hhbc(Hhbc::LateBoundCls(loc))),
            SpecialClsRef::ParentCls => builder.emit(Instr::Hhbc(Hhbc::ParentCls(loc))),
            _ => unreachable!(),
        }
    }
}

impl TransformInstr for LowerInstrs<'_> {
    fn apply(
        &mut self,
        iid: InstrId,
        instr: Instr,
        builder: &mut FuncBuilder<'_>,
        state: &mut TransformState,
    ) -> Instr {
        use hack::Builtin;

        if let Some(instr) = self.handle_with_builtin(builder, &instr) {
            self.changed = true;
            return instr;
        }

        let instr = match instr {
            // NOTE: Be careful trying to lower most Instr::Call - the details
            // matter and it's hard to rewrite them to preserve those details
            // properly.
            Instr::Call(
                box mut call @ Call {
                    detail: CallDetail::FCallCtor,
                    ..
                },
            ) => {
                let flavor = ObjMethodOp::NullThrows;
                let method = MethodId::from_str("__construct", &builder.strings);
                call.detail = CallDetail::FCallObjMethodD { flavor, method };
                Instr::Call(Box::new(call))
            }
            Instr::Call(
                box call @ Call {
                    detail:
                        CallDetail::FCallObjMethod {
                            flavor: ObjMethodOp::NullSafe,
                            ..
                        }
                        | CallDetail::FCallObjMethodD {
                            flavor: ObjMethodOp::NullSafe,
                            ..
                        },
                    ..
                },
            ) => rewrite_nullsafe_call(iid, builder, call, state),
            Instr::Hhbc(Hhbc::BareThis(_op, loc)) => {
                let this = builder.strings.intern_str("$this");
                let lid = LocalId::Named(this);
                Instr::Hhbc(Hhbc::CGetL(lid, loc))
            }
            Instr::Hhbc(Hhbc::CheckThis(loc)) => {
                let builtin = hack::Hhbc::CheckThis;
                let op = BareThisOp::NoNotice;
                let this = builder.emit(Instr::Hhbc(Hhbc::BareThis(op, loc)));
                builder.hhbc_builtin(builtin, &[this], loc)
            }
            Instr::Hhbc(Hhbc::ClsCns(cls, const_id, loc)) => {
                let builtin = hack::Hhbc::ClsCns;
                let const_id = builder.emit_constant(Constant::String(const_id.id));
                builder.hhbc_builtin(builtin, &[cls, const_id], loc)
            }
            Instr::Hhbc(Hhbc::InstanceOfD(vid, cid, loc)) => {
                let cid = builder.emit_constant(Constant::String(cid.id));
                builder.hack_builtin(Builtin::IsType, &[vid, cid], loc)
            }
            Instr::Hhbc(Hhbc::IsTypeStructC([obj, ts], op, loc)) => {
                let builtin = hack::Hhbc::IsTypeStructC;
                let op = match op {
                    TypeStructResolveOp::DontResolve => 0,
                    TypeStructResolveOp::Resolve => 1,
                    _ => unreachable!(),
                };
                let op = builder.emit_constant(Constant::Int(op));
                builder.hhbc_builtin(builtin, &[obj, ts, op], loc)
            }
            Instr::Hhbc(Hhbc::LateBoundCls(loc)) => {
                match self.method_info.as_ref().unwrap().is_static {
                    IsStatic::Static => {
                        let this = builder.emit(Instr::Hhbc(Hhbc::This(loc)));
                        builder.hack_builtin(Builtin::GetClass, &[this], loc)
                    }
                    IsStatic::NonStatic => {
                        let this = builder.emit(Instr::Hhbc(Hhbc::This(loc)));
                        builder.hack_builtin(Builtin::GetStaticClass, &[this], loc)
                    }
                }
            }
            Instr::Hhbc(Hhbc::LazyClass(clsid, _)) => {
                // Treat a lazy class as a simple string.
                let c = builder.emit_constant(Constant::String(clsid.id));
                Instr::copy(c)
            }
            Instr::Hhbc(Hhbc::LockObj(obj, loc)) => {
                builder.emit_hhbc_builtin(hack::Hhbc::LockObj, &[obj], loc);
                Instr::copy(obj)
            }
            Instr::Hhbc(Hhbc::NewStructDict(names, values, loc)) => {
                let args = names
                    .iter()
                    .zip(values.iter().copied())
                    .flat_map(|(name, value)| {
                        [builder.emit_constant(Constant::String(*name)), value]
                    })
                    .collect_vec();
                builder.hack_builtin(Builtin::NewDict, &args, loc)
            }
            Instr::Hhbc(Hhbc::NewObj(cls, loc)) => {
                let method = MethodId::from_str("__factory", &builder.strings);
                let operands = vec![cls].into_boxed_slice();
                let context = UnitBytesId::NONE;
                let flavor = ObjMethodOp::NullThrows;
                let detail = CallDetail::FCallObjMethodD { flavor, method };
                let flags = FCallArgsFlags::default();
                Instr::Call(Box::new(Call {
                    operands,
                    context,
                    detail,
                    flags,
                    num_rets: 1,
                    inouts: None,
                    readonly: None,
                    loc,
                }))
            }
            Instr::Hhbc(Hhbc::NewObjD(clsid, loc)) => {
                let cls = builder.emit(Instr::Hhbc(Hhbc::ResolveClass(clsid, loc)));
                let method = MethodId::from_str("__factory", &builder.strings);
                let operands = vec![cls].into_boxed_slice();
                let context = UnitBytesId::NONE;
                let flavor = ObjMethodOp::NullThrows;
                let detail = CallDetail::FCallObjMethodD { flavor, method };
                let flags = FCallArgsFlags::default();
                Instr::Call(Box::new(Call {
                    operands,
                    context,
                    detail,
                    flags,
                    num_rets: 1,
                    inouts: None,
                    readonly: None,
                    loc,
                }))
            }
            Instr::Hhbc(Hhbc::NewObjS(clsref, loc)) => {
                let cls = self.emit_special_cls_ref(builder, clsref, loc);
                Instr::Hhbc(Hhbc::NewObj(cls, loc))
            }
            Instr::Hhbc(Hhbc::VerifyOutType(vid, lid, loc)) => {
                self.verify_out_type(builder, vid, lid, loc)
            }
            Instr::Hhbc(Hhbc::VerifyParamTypeTS(ts, lid, loc)) => {
                self.verify_param_type_ts(builder, lid, ts, loc)
            }
            Instr::Hhbc(Hhbc::VerifyRetTypeC(vid, loc)) => {
                self.verify_ret_type_c(builder, vid, loc)
            }
            Instr::Hhbc(Hhbc::VerifyRetTypeTS([obj, ts], loc)) => {
                self.verify_ret_type_ts(builder, obj, ts, loc)
            }
            Instr::Hhbc(Hhbc::VerifyImplicitContextState(_)) => {
                // no-op
                Instr::tombstone()
            }
            Instr::Hhbc(Hhbc::IterFree(id, loc)) => {
                let lid = iter_var_name(id, &builder.strings);
                let value = builder.emit(Instr::Hhbc(Hhbc::CGetL(lid, loc)));
                builder.hhbc_builtin(hack::Hhbc::IterFree, &[value], loc)
            }
            Instr::Terminator(Terminator::IterInit(args, value)) => {
                self.iter_init(builder, args, value)
            }
            Instr::Terminator(Terminator::IterNext(args)) => self.iter_next(builder, args),
            Instr::Terminator(Terminator::RetM(ops, loc)) => {
                // ret a, b;
                // =>
                // ret vec[a, b];
                let builtin = hack::Hhbc::NewVec;
                let vec = builder.emit_hhbc_builtin(builtin, &ops, loc);
                Instr::ret(vec, loc)
            }
            Instr::Terminator(Terminator::SSwitch {
                cond,
                cases,
                targets,
                loc,
            }) => {
                // if (StrEq(cond, case_0)) jmp targets[0];
                // else if (StrEq(cond, case_1)) jmp targets[1];
                // ...
                // else jmp targets.last();

                // Last case MUST be 'default'.
                let iter = cases.iter().take(cases.len() - 1).zip(targets.iter());

                for (case, target) in iter {
                    let string = builder.emit_constant(Constant::String(*case));
                    let pred = builder.emit_hhbc_builtin(hack::Hhbc::CmpSame, &[string, cond], loc);
                    let false_bid = builder.alloc_bid();
                    builder.emit(Instr::jmp_op(
                        pred,
                        Predicate::NonZero,
                        *target,
                        false_bid,
                        loc,
                    ));
                    builder.start_block(false_bid);
                }

                Instr::jmp(*targets.last().unwrap(), loc)
            }
            instr => {
                return instr;
            }
        };
        self.changed = true;
        instr
    }
}

fn rewrite_nullsafe_call(
    original_iid: InstrId,
    builder: &mut FuncBuilder<'_>,
    mut call: Call,
    state: &mut TransformState,
) -> Instr {
    // rewrite a call like `$a?->b()` into `$a ? $a->b() : null`
    call.detail = match call.detail {
        CallDetail::FCallObjMethod { .. } => CallDetail::FCallObjMethod {
            flavor: ObjMethodOp::NullThrows,
        },
        CallDetail::FCallObjMethodD { method, .. } => CallDetail::FCallObjMethodD {
            flavor: ObjMethodOp::NullThrows,
            method,
        },
        _ => unreachable!(),
    };

    // We need to be careful about multi-return!
    //   (ret, a, b) = obj?->call(inout a, inout b)
    // ->
    //   if (obj) {
    //     (ret, a, b)
    //   } else {
    //     (null, a, b)
    //   }

    let loc = call.loc;
    let obj = call.obj();
    let num_rets = call.num_rets;
    let pred = builder.emit_hack_builtin(hack::Builtin::Hhbc(hack::Hhbc::IsTypeNull), &[obj], loc);

    // Need to customize the if/then/else because of multi-return.
    let join_bid = builder.alloc_bid();
    let null_bid = builder.alloc_bid();
    let nonnull_bid = builder.alloc_bid();
    builder.emit(Instr::jmp_op(
        pred,
        Predicate::NonZero,
        null_bid,
        nonnull_bid,
        loc,
    ));

    // Null case - main value should be null. Inouts should be passed through.
    builder.start_block(null_bid);
    let mut args = Vec::with_capacity(num_rets as usize);
    args.push(builder.emit_constant(Constant::Null));
    if let Some(inouts) = call.inouts.as_ref() {
        for inout_idx in inouts.iter() {
            let op = call.operands[*inout_idx as usize];
            args.push(op);
        }
    }
    builder.emit(Instr::jmp_args(join_bid, &args, loc));

    // Nonnull case - make call and pass Select values.
    builder.start_block(nonnull_bid);
    args.clear();
    let iid = builder.emit(Instr::Call(Box::new(call)));
    if num_rets > 1 {
        for idx in 0..num_rets {
            args.push(builder.emit(Instr::Special(Special::Select(iid, idx))));
        }
    } else {
        args.push(iid);
    }
    builder.emit(Instr::jmp_args(join_bid, &args, loc));

    // Join
    builder.start_block(join_bid);
    args.clear();
    if num_rets > 1 {
        // we need to swap out the original Select statements.
        for idx in 0..num_rets {
            let param = builder.alloc_param();
            args.push(param);
            state.rewrite_select_idx(original_iid, idx as usize, param);
        }
        Instr::tombstone()
    } else {
        let param = builder.alloc_param();
        args.push(param);
        Instr::copy(args[0])
    }
}

fn iter_var_name(id: ir::IterId, strings: &ir::StringInterner) -> LocalId {
    let name = strings.intern_str(format!("iter{}", id.idx));
    LocalId::Named(name)
}
