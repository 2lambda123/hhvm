// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.
use crate::decl_defs::{CeVisibility, DeclTy};
use crate::reason::Reason;
use hcons::Hc;
use pos::Symbol;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FoldedElement<R: Reason> {
    // note(sf, 2022-01-28): c.f. `Decl_defs.element`
    pub origin: Symbol,
    pub visibility: CeVisibility<R>,
    pub deprecated: Option<Hc<str>>,
}

#[derive(Debug, Clone)]
pub struct SubstContext<R: Reason> {
    // note(sf, 2022-01-28): c.f. `Decl_defs.subst_context`
    pub subst: HashMap<Symbol, DeclTy<R>>,
    pub class_context: Symbol,
}

#[derive(Debug, Clone)]
pub struct FoldedClass<R: Reason> {
    // note(sf, 2022-01-27): c.f. `Decl_defs.decl_class_type`
    pub name: Symbol,
    pub pos: R::Pos,
    pub substs: HashMap<Symbol, SubstContext<R>>,
    pub ancestors: HashMap<Symbol, DeclTy<R>>,
    pub methods: HashMap<Symbol, FoldedElement<R>>,
    pub static_methods: HashMap<Symbol, FoldedElement<R>>,
}
