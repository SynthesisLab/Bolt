//! Types used for Boolean Formulas
pub(crate) mod cache;
pub(crate) mod charac;
pub(crate) mod cv;
pub(crate) mod hash;
pub(crate) mod sv;

use charac::BoolCharac;

use super::formula::Formula;

pub type BoolFormula = Formula<BoolCharac>;

impl BoolFormula {
    pub(crate) fn sat_positive_count(&self) -> usize {
        self.charac.sat_positive_count()
    }

    pub(crate) fn sat_negative_count(&self) -> usize {
        self.charac.sat_negative_count()
    }
}
