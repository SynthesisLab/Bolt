//! Types used for LTL Formulas
use charac::LtlCharac;

use super::formula::Formula;

pub mod cache;
pub mod charac;
pub mod cm;
pub mod cs;
pub mod hash;
pub mod trace;

/// Represents an atomic proposition, e.g. `p`, `q`, `var0`, etc.
///
/// The first component is the string representation of the proposition,
/// the second is its index in the vector of atomic propositions.
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct AtomicProposition(pub(crate) String, pub(crate) usize);

pub(crate) type LtlFormula = Formula<LtlCharac>;

impl LtlFormula {
    pub(crate) fn accepted_vec(&self) -> Vec<bool> {
        self.charac.cm.accepted_vec()
    }
}
