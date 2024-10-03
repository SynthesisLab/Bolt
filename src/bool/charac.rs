//! Characteristic representation of a boolean formula.
//!
//! A [`BoolCharac`] is a [characteristic vector][`CharVec`] with the corresponding
//! [satisfiability vector][`SatVec`] and [hash](BoolHash) stored to avoid recomputing them.
use crate::{
    ops::{
        binary::LtlBinaryOp,
        traits::{BinaryOp, UnaryOp},
        unary::LtlUnaryOp,
    },
    traits::{EqTarget, Hashed},
};

use super::{cv::CharVec, hash::BoolHash, sv::SatVec};

/// Represents a [characteristic vector][`CharVec`] with the corresponding
/// [satisfiability vector][`SatVec`] and [hash](BoolHash) stored to avoid recomputing them.
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct BoolCharac {
    pub(crate) cv: CharVec,
    pub(crate) sv: SatVec,
    cv_hash: BoolHash,
}

impl BoolCharac {
    pub(crate) fn from_cv(cv: CharVec, target: CharVec) -> Self {
        let sv = cv.satisfied(target);
        let cv_hash = cv.hashed();
        BoolCharac { cv, sv, cv_hash }
    }

    pub(crate) fn sat_positive_count(&self) -> usize {
        let v = self.cv.values & self.sv.values;
        v.count_ones() as usize
    }

    pub(crate) fn sat_negative_count(&self) -> usize {
        let v = (!self.cv.values) & self.sv.values;
        v.count_ones() as usize
    }
}

impl UnaryOp for BoolCharac {
    fn apply(_op: LtlUnaryOp, _f: &Self) -> Self {
        unimplemented!("Never used, only needed for trait bounds")
    }
}

impl BinaryOp for BoolCharac {
    fn apply(op: LtlBinaryOp, f1: &Self, f2: &Self) -> Self {
        let cv = LtlBinaryOp::apply_cv(op, f1.cv, f2.cv);
        let not_target = f1.cv.xor_satvec(f1.sv);
        let sv = cv.xor_satvec(not_target);
        let cv_hash = cv.hashed();
        Self { cv, sv, cv_hash }
    }
}

impl Hashed for BoolCharac {
    type HashType = BoolHash;

    // Note: it is more efficient to store the hash of the `cv` and use it for the equivalence test
    // instead of using the `cv` directly, as the `cv` contains a [`u128`] which take more time to hash.
    fn hashed(&self) -> Self::HashType {
        self.cv_hash
    }
}

impl EqTarget for BoolCharac {
    type TargetType = CharVec;

    fn eq_target(&self, target: &Self::TargetType) -> bool {
        self.cv == *target
    }
}
