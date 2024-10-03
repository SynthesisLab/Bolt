//! Satisfiability vector of boolean formulas, represented as bit vectors.
use std::{
    fmt::{Debug, Display},
    ops::Not,
};

/// Satisfiability vector of a boolean formula.
///
/// Defined by `self.values[i] == 1` if and only if
/// the corresponding formula satisfies the ith input.
/// I.e. the input is positive and formula is true
/// or the input is negative and formula is false.
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct SatVec {
    pub(super) values: u128,
}

impl SatVec {
    pub(crate) fn popcount(&self) -> u32 {
        self.values.count_ones()
    }

    /// Whether `self` dominates `other`.
    pub(crate) fn dominates(&self, other: Self) -> bool {
        // Other.values is a subset of self.values
        // iff the intersection of other.values
        // and the complement of self.values is empty.
        self.values.not() & other.values == 0
    }
}

impl Debug for SatVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl Display for SatVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = self.values;
        for i in 0..128 {
            write!(f, "{}", (x >> i) & 1)?;
        }
        Ok(())
    }
}
