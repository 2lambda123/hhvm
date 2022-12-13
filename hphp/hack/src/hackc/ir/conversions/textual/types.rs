// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.

use ir::StringInterner;
use ir::TypeConstraintFlags;

use crate::class::IsStatic;
use crate::mangle::MangleClass as _;
use crate::textual;

pub(crate) fn convert_ty(ty: &ir::EnforceableType, strings: &StringInterner) -> textual::Ty {
    let mut base = convert_base(&ty.ty, strings);

    let mut modifiers = ty.modifiers;

    // ExtendedHint does nothing interesting.
    modifiers -= TypeConstraintFlags::ExtendedHint;
    // DisplayNullable does nothing interesting.
    modifiers -= TypeConstraintFlags::DisplayNullable;

    // All textual boxed types are nullable.
    modifiers -= TypeConstraintFlags::Nullable;

    if modifiers != TypeConstraintFlags::NoFlags {
        log::trace!("MODIFIERS: {modifiers:?}");
        textual_todo! {
            base = textual::Ty::Type("TODO_NoFlags".to_string());
        }
    }

    base
}

fn convert_base(ty: &ir::BaseType, strings: &StringInterner) -> textual::Ty {
    use ir::BaseType;
    match ty {
        BaseType::Arraykey => textual::Ty::SpecialPtr(textual::SpecialTy::Arraykey),
        BaseType::Bool => textual::Ty::SpecialPtr(textual::SpecialTy::Bool),
        BaseType::Class(cid) => {
            textual::Ty::named_type_ptr(cid.mangle_class(IsStatic::NonStatic, strings))
        }
        BaseType::Classname => textual::Ty::named_type_ptr("Classname".to_owned()),
        BaseType::Dict => textual::Ty::SpecialPtr(textual::SpecialTy::Dict),
        BaseType::Float => textual::Ty::SpecialPtr(textual::SpecialTy::Float),
        BaseType::Int => textual::Ty::SpecialPtr(textual::SpecialTy::Int),
        BaseType::Keyset => textual::Ty::SpecialPtr(textual::SpecialTy::Keyset),
        BaseType::Null => textual::Ty::VoidPtr,
        BaseType::Num => textual::Ty::SpecialPtr(textual::SpecialTy::Num),
        BaseType::Resource => textual::Ty::named_type_ptr("HackResource".to_owned()),
        BaseType::String => textual::Ty::SpecialPtr(textual::SpecialTy::String),
        BaseType::Vec => textual::Ty::SpecialPtr(textual::SpecialTy::Vec),
        // Why VoidPtr? Because in Hack something returning `void` implicitly
        // returns a null.
        BaseType::Void => textual::Ty::VoidPtr,

        BaseType::AnyArray
        | BaseType::Darray
        | BaseType::Varray
        | BaseType::VarrayOrDarray
        | BaseType::VecOrDict => textual::Ty::named_type_ptr("HackArray".to_owned()),

        BaseType::Noreturn | BaseType::Nothing => textual::Ty::Noreturn,

        BaseType::Mixed
        | BaseType::None
        | BaseType::Nonnull
        | BaseType::This
        | BaseType::Typename => textual::Ty::SpecialPtr(textual::SpecialTy::Mixed),
    }
}
