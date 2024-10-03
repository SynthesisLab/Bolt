//! Characteristic representation of an LTL formula.
use crate::{
    ops::{
        binary::LtlBinaryOp,
        traits::{BinaryOp, UnaryOp},
        unary::LtlUnaryOp,
    },
    traits::{EqTarget, Hashed},
};

use super::{cm::CharMatrix, cs::CharSeq, hash::LtlHash};

/// Represents a [characteristic matrix][`CharMatrix`] with the corresponding
/// [hash](LtlHash) stored to avoid recomputing it every time.
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub(crate) struct LtlCharac {
    pub(crate) cm: CharMatrix,
    cm_hash: LtlHash,
}

impl FromIterator<CharSeq> for LtlCharac {
    fn from_iter<T: IntoIterator<Item = CharSeq>>(iter: T) -> Self {
        let cm = iter.into_iter().collect::<CharMatrix>();
        let cm_hash = cm.hashed();
        Self { cm, cm_hash }
    }
}

impl UnaryOp for LtlCharac {
    fn apply(op: LtlUnaryOp, f: &Self) -> Self {
        let cm = LtlUnaryOp::apply_cm(op, &f.cm);
        let cm_hash = cm.hashed();
        Self { cm, cm_hash }
    }
}

impl BinaryOp for LtlCharac {
    fn apply(op: LtlBinaryOp, f1: &Self, f2: &Self) -> Self {
        let cm = LtlBinaryOp::apply_cm(op, &f1.cm, &f2.cm);
        let cm_hash = cm.hashed();
        Self { cm, cm_hash }
    }
}

impl Hashed for LtlCharac {
    type HashType = LtlHash;

    fn hashed(&self) -> Self::HashType {
        self.cm_hash
    }
}

impl EqTarget for LtlCharac {
    type TargetType = [bool];

    fn eq_target(&self, target: &Self::TargetType) -> bool {
        self.cm.is_equivalent(target)
    }
}
