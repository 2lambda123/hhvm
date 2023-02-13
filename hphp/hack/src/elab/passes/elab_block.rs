// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.
use std::collections::VecDeque;
use std::ops::ControlFlow;

use oxidized::aast_defs::Block;
use oxidized::aast_defs::Stmt;
use oxidized::aast_defs::Stmt_;
use oxidized::aast_defs::UsingStmt;
use oxidized::naming_phase_error::NamingPhaseError;

use crate::config::Config;
use crate::Pass;

#[derive(Clone, Copy, Default)]
pub struct ElabBlockPass;

impl Pass for ElabBlockPass {
    type Cfg = Config;
    type Err = NamingPhaseError;

    fn on_ty_block_top_down<Ex: Default, En>(
        &mut self,
        elem: &mut Block<Ex, En>,
        _cfg: &Self::Cfg,
        _errs: &mut Vec<Self::Err>,
    ) -> ControlFlow<(), ()> {
        let mut q: VecDeque<_> = elem.drain(0..).collect();
        while let Some(Stmt(pos, stmt_)) = q.pop_front() {
            match stmt_ {
                Stmt_::Block(xs) => xs.into_iter().rev().for_each(|x| q.push_front(x)),
                _ => elem.push(Stmt(pos, stmt_)),
            }
        }
        ControlFlow::Continue(())
    }

    fn on_ty_using_stmt_top_down<Ex: Default, En>(
        &mut self,
        elem: &mut UsingStmt<Ex, En>,
        _cfg: &Self::Cfg,
        _errs: &mut Vec<Self::Err>,
    ) -> ControlFlow<(), ()> {
        elem.is_block_scoped = false;
        ControlFlow::Continue(())
    }
}

#[cfg(test)]
mod tests {

    use oxidized::aast_defs::Block;
    use oxidized::aast_defs::Stmt;
    use oxidized::aast_defs::Stmt_;
    use oxidized::tast::Pos;

    use super::*;
    use crate::Transform;

    #[test]
    fn test() {
        let cfg = Config::default();
        let mut errs = Vec::default();
        let mut pass = ElabBlockPass;

        let mut elem: Block<(), ()> = Block(vec![Stmt(
            Pos::make_none(),
            Stmt_::Block(Block(vec![
                Stmt(Pos::make_none(), Stmt_::Noop),
                Stmt(
                    Pos::make_none(),
                    Stmt_::Block(Block(vec![
                        Stmt(Pos::make_none(), Stmt_::Noop),
                        Stmt(
                            Pos::make_none(),
                            Stmt_::Block(Block(vec![
                                Stmt(Pos::make_none(), Stmt_::Noop),
                                Stmt(
                                    Pos::make_none(),
                                    Stmt_::Block(Block(vec![
                                        Stmt(Pos::make_none(), Stmt_::Noop),
                                        Stmt(
                                            Pos::make_none(),
                                            Stmt_::Block(Block(vec![Stmt(
                                                Pos::make_none(),
                                                Stmt_::Noop,
                                            )])),
                                        ),
                                        Stmt(Pos::make_none(), Stmt_::Noop),
                                    ])),
                                ),
                                Stmt(Pos::make_none(), Stmt_::Noop),
                            ])),
                        ),
                        Stmt(Pos::make_none(), Stmt_::Noop),
                    ])),
                ),
                Stmt(Pos::make_none(), Stmt_::Noop),
            ])),
        )]);
        elem.transform(&cfg, &mut errs, &mut pass);

        assert_eq!(elem.len(), 9);
        assert!(elem.into_iter().all(|s| matches!(s.1, Stmt_::Noop)));
    }
}
