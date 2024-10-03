//! (Meta-)Algorithms for LTL and Boolean Synthesis.
//!
//! Meta-algorithms first run an LTL enumeration algorithm for a fixed number of iterations,
//! if no solution is found, they run a Boolean Synthesis on the set of LTL formulas to
//! find a solution.
//!
//! This module contains the following meta-algorithms:
//! - [Divide and conquer](self::meta)
//!
//! This module contains the following Boolean Synthesis algorithms:
//! - [Set Cover](self::set_cover)
//! - [Semantic Enumeration](self::enumeration)
//! - [Beam Search](self::beam_search)
//!
//! Implementing a Boolean Synthesis for use with meta-algorithms is done via
//! the [`BoolAlgoParams`] trait.
use std::{ops::Not, rc::Rc};

use meta::cache::InitialBoolCache;

use crate::{
    cache::{EnumFormulaCache, EnumFormulaCacheLine},
    formula::{tree::FormulaTree, Formula},
    ltl::{
        cache::LtlCache,
        charac::LtlCharac,
        trace::{Operators, Trace},
        LtlFormula, Predicate, PredicateForm,
    },
    traits::EqTarget,
};

pub mod beam_search;
pub mod enumeration;
pub mod meta;
pub mod set_cover;

/// Abstraction for the hyperparameters of Boolean Synthesis algo, used to launch multiple runs.
pub trait BoolAlgoParams {
    /// Additional info returned by a run, e.g. for collecting data for experiments.
    /// If unneeded, just use `()`.
    type Data;
    /// Run the algorithm with the contained hyperparameters.
    fn run(
        &self,
        cache: InitialBoolCache,
        operators: Operators,
        target: &[bool],
    ) -> (Option<FormulaTree>, Self::Data);
    /// Nickname of the algorithm, used for logging results.
    fn name() -> &'static str;
}

/// Return a [`Vec`] containing all size-1 LTL formulas: the predicates and their negation.
fn atoms(traces: &[Trace], alphabet: Vec<String>) -> Vec<LtlFormula> {
    let mut atoms = Vec::new();
    for (i, s) in alphabet.into_iter().enumerate() {
        let charac = traces.iter().map(|t| t.alphabet[i]).collect::<LtlCharac>();
        let f = Formula::new_base(
            charac,
            1,
            Rc::from(FormulaTree::Atom(Predicate(
                s.clone(),
                PredicateForm::Positive(i),
            ))),
        );
        atoms.push(f);

        let charac = traces
            .iter()
            .map(|t| t.alphabet[i].not())
            .collect::<LtlCharac>();
        let not_f = Formula::new_base(
            charac,
            1,
            Rc::from(FormulaTree::Atom(Predicate(
                format!("!{s}"),
                PredicateForm::Negative(i),
            ))),
        );
        atoms.push(not_f);
    }

    atoms
}

/// Create an [`LtlCache`] containing all formulas in `atoms`.
fn create_initial_cache(atoms: Vec<LtlFormula>, target: &[bool]) -> (Option<LtlFormula>, LtlCache) {
    let mut ltl_cache = LtlCache::new();
    // Add empty line for size 0 in cache
    ltl_cache.new_line(0);

    let mut initial_line = ltl_cache.new_line(1);
    for f in atoms {
        initial_line.push(f);
    }

    let found_atom = ltl_cache.iter_size(1).find_map(|f| {
        if f.eq_target(target) {
            Some(f.clone())
        } else {
            None
        }
    });

    (found_atom, ltl_cache)
}
