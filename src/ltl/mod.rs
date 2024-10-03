//! Types used for LTL Formulas
use charac::LtlCharac;

use super::formula::Formula;

pub mod cache;
pub mod charac;
pub mod cm;
pub mod cs;
pub mod hash;
pub mod trace;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Predicate(pub(crate) String, pub(crate) PredicateForm);

/// Formula corresponding to a single variable `x_i`, which may be negated.
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum PredicateForm {
    /// Formula `x_i`
    Positive(usize),
    /// Formula `not x_i`
    Negative(usize),
}

pub(crate) type LtlFormula = Formula<LtlCharac>;

impl LtlFormula {
    pub(crate) fn accepted_vec(&self) -> Vec<bool> {
        self.charac.cm.accepted_vec()
    }
}
