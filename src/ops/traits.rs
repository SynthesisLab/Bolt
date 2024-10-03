use super::{binary::LtlBinaryOp, unary::LtlUnaryOp};

pub(crate) trait UnaryOp {
    fn apply(op: LtlUnaryOp, f: &Self) -> Self;
}

/// Trait implemented by types to which binary LTL operators can be applied.
pub(crate) trait BinaryOp {
    fn apply(op: LtlBinaryOp, f1: &Self, f2: &Self) -> Self;
}

/// Whether the operator commutes
pub(crate) trait Commutativity {
    fn commutes(&self) -> bool;
}
