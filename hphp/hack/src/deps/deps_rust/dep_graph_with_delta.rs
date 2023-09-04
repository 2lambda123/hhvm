// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.

use std::ops::Deref;
use std::ops::DerefMut;

use dep_graph_delta::DepGraphDelta;
use dep_graph_delta::HashSetDelta;
use depgraph_reader::BaseDepgraphTrait;
use depgraph_reader::Dep;
use depgraph_reader::DepGraph;
use itertools::Either;
use once_cell::sync::OnceCell;
use parking_lot::RwLock;

use crate::RawTypingDepsMode;
use crate::TypingDepsMode;

pub trait DependentIterator {
    fn iter_dependents_with_duplicates<F, R>(&self, mode: RawTypingDepsMode, dep: Dep, f: F) -> R
    where
        F: FnMut(&mut dyn Iterator<Item = Dep>) -> R;
}

pub struct DepGraphWithDelta<'a, B> {
    base: Option<&'a B>,
    delta: &'a DepGraphDelta,
}

impl<'a, B: BaseDepgraphTrait> DependentIterator for DepGraphWithDelta<'a, B> {
    /// Iterates over all dependents of a dependency, with possibly duplicate dependents.
    fn iter_dependents_with_duplicates<F, R>(
        &self,
        _mode: RawTypingDepsMode,
        dep: Dep,
        mut f: F,
    ) -> R
    where
        F: FnMut(&mut dyn Iterator<Item = Dep>) -> R,
    {
        let Self { base, delta } = self;
        let HashSetDelta { added, removed } = delta.get(dep);
        let mut added_iter = added
            .map(|s| s.iter().copied())
            .map_or(Either::Right(std::iter::empty::<Dep>()), Either::Left);
        let is_removed = |d: &Dep| match removed {
            None => false,
            Some(removed) => removed.contains(d),
        };
        match base {
            None => f(&mut added_iter),
            Some(base) => {
                let base_iter = base.iter_dependents(dep);
                f(&mut added_iter.chain(base_iter.filter(|d| !is_removed(d))))
            }
        }
    }
}

impl<'a, B: BaseDepgraphTrait> DepGraphWithDelta<'a, B> {
    pub fn new(base: Option<&'a B>, delta: &'a DepGraphDelta) -> Self {
        Self { base, delta }
    }

    /// Returns true if we know for sure that the depgraph has the edge, false
    /// if we don't know.
    pub fn has_edge_for_sure(&self, dependent: Dep, dependency: Dep) -> bool {
        let Self { base, delta } = self;
        base.map_or(false, |g| {
            g.dependent_dependency_edge_exists(dependent, dependency)
        }) && !delta.edge_is_removed(dependent, dependency)
    }
}

pub struct LockedDepgraphWithDelta {
    /// A structure wrapping the memory-mapped dependency graph.
    /// Each worker will itself lazily (or eagerly upon request)
    /// open a memory-mapping to the dependency graph.
    ///
    /// It's an option, because custom mode might be enabled without
    /// an existing saved-state.
    base: RwLock<Option<DepGraph>>,
    /// The dependency graph delta.
    ///
    /// Even though this is only used in a single-threaded context (from OCaml)
    /// we wrap it in a `Mutex` to ensure safety.
    delta: OnceCell<RwLock<DepGraphDelta>>,
}

impl LockedDepgraphWithDelta {
    pub const fn new(
        base: RwLock<Option<DepGraph>>,
        delta: OnceCell<RwLock<DepGraphDelta>>,
    ) -> Self {
        Self { base, delta }
    }

    pub fn base(&self) -> &RwLock<Option<DepGraph>> {
        &self.base
    }

    pub fn delta(&self) -> &RwLock<DepGraphDelta> {
        self.delta.get_or_init(Default::default)
    }

    /// Run the closure with the dep graph delta.
    ///
    /// # Panics
    ///
    /// When another reference to delta is still active, but that
    /// isn't likely,given that we only have one thread, and the
    /// `with`/`with_mut` auxiliary functions disallow the reference
    /// to escape.
    pub fn lock_delta_and<R>(&self, f: impl FnOnce(&DepGraphDelta) -> R) -> R {
        f(self.delta().read().deref())
    }

    /// Run the closure with the mutable dep graph delta.
    ///
    /// # Panics
    ///
    /// See `with`
    pub fn lock_mut_delta_and<R>(&self, f: impl FnOnce(&mut DepGraphDelta) -> R) -> R {
        f(self.delta().write().deref_mut())
    }

    /// Load the graph using the given mode.
    ///
    /// The mode is only used on the first call, to establish some global state, and
    /// then ignored for future calls.
    ///
    /// # Safety
    ///
    /// The pointer to the dependency graph mode should still be pointing
    /// to a valid OCaml object.
    fn load_base(&self, mode: RawTypingDepsMode) -> Result<(), String> {
        let mut dep_graph_guard = self.base().write();

        if dep_graph_guard.is_none() {
            *dep_graph_guard = Self::base_dep_graph_from_mode(mode)?;
        }

        Ok(())
    }

    /// Override the loaded dep graph.
    ///
    /// # Panics
    ///
    /// Panics if the graph is not loaded, and custom mode was not enabled.
    ///
    /// Panics if the graph is not yet loaded, and opening
    /// the graph results in an error.
    ///
    /// # Safety
    ///
    /// The pointer to the dependency graph mode should still be pointing
    /// to a valid OCaml object.
    pub fn replace_dep_graph(&self, mode: RawTypingDepsMode) -> Result<(), String> {
        let mut dep_graph_guard = self.base().write();
        *dep_graph_guard = Self::base_dep_graph_from_mode(mode)?;
        Ok(())
    }

    fn base_dep_graph_from_mode(mode: RawTypingDepsMode) -> Result<Option<DepGraph>, String> {
        // # Safety
        //
        // The pointer to the dependency graph mode should still be pointing
        // to a valid OCaml object.
        let mode = unsafe { mode.to_rust().unwrap() };

        match mode {
            TypingDepsMode::InMemoryMode(None)
            | TypingDepsMode::SaveToDiskMode {
                graph: None,
                new_edges_dir: _,
                human_readable_dep_map_dir: _,
            } => {
                // Enabled, but we don't have a saved-state, so we can't open it
                Ok(None)
            }
            TypingDepsMode::InMemoryMode(Some(depgraph_fn))
            | TypingDepsMode::SaveToDiskMode {
                graph: Some(depgraph_fn),
                new_edges_dir: _,
                human_readable_dep_map_dir: _,
            } => {
                // We are opening and intializing the dep graph while holding onto the mutex...
                // Which typically isn't great, but since ocaml is single threaded, it's ok.
                let depgraph = DepGraph::from_path(depgraph_fn)
                    .map_err(|err| format!("could not open dep graph file: {:?}", err))?;
                Ok(Some(depgraph))
            }
            TypingDepsMode::HhFanoutRustMode { hh_fanout: _ } => {
                // HhFanoutRustMode doesn't load the dep graph this way.
                // This path shouldn't be reached.
                unimplemented!()
            }
        }
    }

    /// Run the closure with the loaded dep graph. If the custom dep graph
    /// mode was enabled without a saved-state, the closure is run without
    /// a dep graph.
    ///
    /// The mode is only used on the first call, to establish some global state, and
    /// then ignored for future calls.
    ///
    /// # Panics
    ///
    /// Panics if the graph is not loaded, and custom mode was not enabled.
    ///
    /// Panics if the graph is not yet loaded, and opening
    /// the graph results in an error.
    ///
    /// # Safety
    ///
    /// The pointer to the dependency graph mode should still be pointing
    /// to a valid OCaml object.
    pub fn lock_base_and<F, R>(&self, mode: RawTypingDepsMode, f: F) -> R
    where
        F: FnOnce(Option<&DepGraph>) -> R,
    {
        self.load_base(mode).unwrap();
        f(self.base().read().as_ref())
    }

    pub fn lock_and<F, R>(&self, mode: RawTypingDepsMode, f: F) -> R
    where
        F: FnOnce(DepGraphWithDelta<'_, DepGraph>) -> R,
    {
        self.lock_delta_and(|delta| {
            self.lock_base_and(mode, |g| f(DepGraphWithDelta::new(g, delta)))
        })
    }
}

impl DependentIterator for LockedDepgraphWithDelta {
    /// Iterates over all dependents of a dependency, with possibly duplicate dependents.
    fn iter_dependents_with_duplicates<F, R>(&self, mode: RawTypingDepsMode, dep: Dep, f: F) -> R
    where
        F: FnMut(&mut dyn Iterator<Item = Dep>) -> R,
    {
        self.lock_and(mode, |g| g.iter_dependents_with_duplicates(mode, dep, f))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::collections::HashSet;

    use super::*;

    pub struct SimpleDepGraph {
        graph: HashMap<Dep, HashSet<Dep>>,
    }

    impl<const N: usize> From<[(Dep, HashSet<Dep>); N]> for SimpleDepGraph {
        fn from(arr: [(Dep, HashSet<Dep>); N]) -> Self {
            Self {
                graph: arr.into_iter().collect(),
            }
        }
    }

    impl BaseDepgraphTrait for SimpleDepGraph {
        fn iter_dependents(&self, dep: Dep) -> Box<dyn Iterator<Item = Dep> + '_> {
            Box::new(
                self.graph
                    .get(&dep)
                    .map_or(Either::Left(std::iter::empty()), |set| {
                        Either::Right(set.iter().copied())
                    }),
            )
        }

        fn dependent_dependency_edge_exists(&self, dependent: Dep, dependency: Dep) -> bool {
            self.graph
                .get(&dependency)
                .map_or(false, |set| set.contains(&dependent))
        }
    }

    static MODE: RawTypingDepsMode = RawTypingDepsMode::dummy_for_test();

    #[test]
    fn test_some_base() {
        let base = SimpleDepGraph::from([
            (Dep::new(0), HashSet::from([Dep::new(1), Dep::new(2)])),
            (Dep::new(3), HashSet::from([Dep::new(2)])),
        ]);
        let mut delta = DepGraphDelta::default();
        delta.insert(Dep::new(4), Dep::new(3));
        delta.insert(Dep::new(3), Dep::new(0));
        delta.remove(Dep::new(2), Dep::new(0));
        let dg = DepGraphWithDelta::new(Some(&base), &delta);

        assert_eq!(
            dg.iter_dependents_with_duplicates(MODE, Dep::new(0), |iter| iter
                .collect::<HashSet<_>>()),
            HashSet::from([Dep::new(1), Dep::new(3)])
        );
        assert_eq!(
            dg.iter_dependents_with_duplicates(MODE, Dep::new(3), |iter| iter
                .collect::<HashSet<_>>()),
            HashSet::from([Dep::new(2), Dep::new(4)])
        );
        assert!(!dg.has_edge_for_sure(Dep::new(2), Dep::new(0)));
        assert!(dg.has_edge_for_sure(Dep::new(2), Dep::new(3)));
    }

    #[test]
    fn test_no_base() {
        let mut delta = DepGraphDelta::default();
        delta.insert(Dep::new(4), Dep::new(3));
        delta.insert(Dep::new(3), Dep::new(0));
        delta.remove(Dep::new(2), Dep::new(0));
        let dg = DepGraphWithDelta::<SimpleDepGraph>::new(None, &delta);

        assert_eq!(
            dg.iter_dependents_with_duplicates(MODE, Dep::new(0), |iter| iter
                .collect::<HashSet<_>>()),
            HashSet::from([Dep::new(3)])
        );
        assert_eq!(
            dg.iter_dependents_with_duplicates(MODE, Dep::new(3), |iter| iter
                .collect::<HashSet<_>>()),
            HashSet::from([Dep::new(4)])
        );
        assert!(!dg.has_edge_for_sure(Dep::new(2), Dep::new(0)));
        assert!(!dg.has_edge_for_sure(Dep::new(2), Dep::new(3)));
    }
}
