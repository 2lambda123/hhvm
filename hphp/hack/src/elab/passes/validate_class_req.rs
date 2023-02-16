// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.

use std::ops::ControlFlow;

use oxidized::aast::Hint;
use oxidized::aast::RequireKind;
use oxidized::aast_defs::ClassReq;
use oxidized::aast_defs::Class_;
use oxidized::ast_defs::ClassishKind;
use oxidized::naming_error::NamingError;
use oxidized::naming_phase_error::NamingPhaseError;

use crate::config::Config;
use crate::Pass;

#[derive(Clone, Copy, Default)]
pub struct ValidateClassReqPass;

impl Pass for ValidateClassReqPass {
    fn on_ty_class__top_down<Ex: Default, En>(
        &mut self,
        cls: &mut Class_<Ex, En>,
        _: &Config,
        errs: &mut Vec<NamingPhaseError>,
    ) -> ControlFlow<(), ()> {
        let is_trait = cls.kind == ClassishKind::Ctrait;
        let is_interface = cls.kind == ClassishKind::Cinterface;
        let find_req = |kind| cls.reqs.iter().find(|&&ClassReq(_, k)| k == kind);
        // `require implements` and `require class` are only allowed in traits.
        if !is_trait {
            if let Some(ClassReq(Hint(pos, _), _)) = find_req(RequireKind::RequireImplements) {
                errs.push(NamingPhaseError::Naming(
                    NamingError::InvalidRequireImplements(pos.clone()),
                ));
            }
            if let Some(ClassReq(Hint(pos, _), _)) = find_req(RequireKind::RequireClass) {
                errs.push(NamingPhaseError::Naming(NamingError::InvalidRequireClass(
                    pos.clone(),
                )));
            }
        }
        // `require extends` is only allowed in traits and interfaces, so if
        // this classish is neither that's an error.
        if !(is_trait || is_interface) {
            if let Some(ClassReq(Hint(pos, _), _)) = find_req(RequireKind::RequireExtends) {
                errs.push(NamingPhaseError::Naming(
                    NamingError::InvalidRequireExtends(pos.clone()),
                ));
            }
        }
        ControlFlow::Continue(())
    }
}
