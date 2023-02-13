// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.

use std::collections::VecDeque;
use std::ops::ControlFlow;

use oxidized::aast_defs::Def;
use oxidized::aast_defs::Program;
use oxidized::aast_defs::Stmt;
use oxidized::aast_defs::Stmt_;
use oxidized::naming_phase_error::NamingPhaseError;

use crate::config::Config;
use crate::Pass;

#[derive(Clone, Copy, Default)]
pub struct ElabDefsPass;

impl Pass for ElabDefsPass {
    fn on_ty_program_top_down<Ex: Default, En>(
        &mut self,
        elem: &mut Program<Ex, En>,
        _cfg: &Config,
        _errs: &mut Vec<NamingPhaseError>,
    ) -> ControlFlow<(), ()> {
        let Program(defs) = elem;
        let mut q: VecDeque<_> = defs.drain(0..).collect();
        while let Some(e) = q.pop_front() {
            match e {
                // Flatten nested namespaces; prepend the elements onto our
                // queue and carry on
                Def::Namespace(ns) => {
                    let (_, ns_defs) = *ns;
                    ns_defs.into_iter().rev().for_each(|x| q.push_front(x))
                }
                // Remove the following top-level definitions
                Def::FileAttributes(_) | Def::NamespaceUse(_) | Def::SetNamespaceEnv(_) => (),
                // Retain the following top-level definitions
                Def::Fun(_)
                | Def::Class(_)
                | Def::Typedef(_)
                | Def::Constant(_)
                | Def::Module(_)
                | Def::SetModule(_) => defs.push(e),
                // Retain all non [Noop] and [Markup] top-level statements
                // note that these statements may still appear in non-top-level
                // positions
                Def::Stmt(ref stmt) => {
                    let Stmt(_, stmt_) = &**stmt;
                    match &stmt_ {
                        Stmt_::Noop => (),
                        Stmt_::Markup(_) => (),
                        _ => defs.push(e),
                    }
                }
            }
        }
        ControlFlow::Continue(())
    }
}

#[cfg(test)]
mod tests {

    use oxidized::aast_defs::Def;
    use oxidized::aast_defs::Program;
    use oxidized::aast_defs::Stmt;
    use oxidized::aast_defs::Stmt_;
    use oxidized::ast_defs::Id;
    use oxidized::tast::Pos;

    use super::*;
    use crate::Transform;

    #[test]
    fn test() {
        let cfg = Config::default();

        let mut elem: Program<(), ()> = Program(vec![
            Def::Stmt(Box::new(Stmt(Pos::make_none(), Stmt_::Break))),
            Def::NamespaceUse(Vec::default()),
            Def::Stmt(Box::new(Stmt(Pos::make_none(), Stmt_::Noop))),
            Def::Namespace(Box::new((
                Id(Pos::make_none(), String::default()),
                vec![
                    Def::NamespaceUse(Vec::default()),
                    Def::Stmt(Box::new(Stmt(Pos::make_none(), Stmt_::Fallthrough))),
                    Def::Stmt(Box::new(Stmt(Pos::make_none(), Stmt_::Noop))),
                    Def::Namespace(Box::new((
                        Id(Pos::make_none(), String::default()),
                        vec![
                            Def::Stmt(Box::new(Stmt(Pos::make_none(), Stmt_::Break))),
                            Def::NamespaceUse(Vec::default()),
                            Def::Stmt(Box::new(Stmt(Pos::make_none(), Stmt_::Noop))),
                        ],
                    ))),
                ],
            ))),
            Def::Stmt(Box::new(Stmt(Pos::make_none(), Stmt_::Fallthrough))),
            Def::NamespaceUse(Vec::default()),
            Def::Stmt(Box::new(Stmt(Pos::make_none(), Stmt_::Noop))),
        ]);

        let mut errs = Vec::default();

        let mut pass = ElabDefsPass;

        elem.transform(&cfg, &mut errs, &mut pass);

        // Given our initial program:
        //
        // [ Break
        // ; NamespaceUse(..)
        // ; Noop
        // ; Namespace(..,
        //     [ NamespaceUse(..)
        //     ; Fallthrough
        //     ; Noop
        //     ; Namespace(..,
        //         [ Break
        //         ; NamespaceUse(..)
        //         ; Noop
        //         ]
        //       )
        //     ]
        //   )
        // ; Fallthrough
        // ; NamespaceUse(..)
        // ; Noop
        // ]
        //
        // We expect the transformed program:
        //
        // [ Break
        // ; Fallthrough
        // ; Break
        // ; Fallthrough
        // ]

        assert_eq!(elem.0.len(), 4);

        let mut q = VecDeque::from(elem.0);

        // First def is Break
        assert!(q.pop_front().is_some_and(|def| match def {
            Def::Stmt(stmt) => match &stmt.1 {
                Stmt_::Break => true,
                _ => false,
            },
            _ => false,
        }));

        // Second def is Fallthrough
        assert!(q.pop_front().is_some_and(|def| match def {
            Def::Stmt(stmt) => match &stmt.1 {
                Stmt_::Fallthrough => true,
                _ => false,
            },
            _ => false,
        }));

        // Third def is Break
        assert!(q.pop_front().is_some_and(|def| match def {
            Def::Stmt(stmt) => match &stmt.1 {
                Stmt_::Break => true,
                _ => false,
            },
            _ => false,
        }));

        // Last def is Fallthrough
        assert!(q.pop_front().is_some_and(|def| match def {
            Def::Stmt(stmt) => match &stmt.1 {
                Stmt_::Fallthrough => true,
                _ => false,
            },
            _ => false,
        }));
    }
}
