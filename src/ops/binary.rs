//! Binary LTL Operators
use std::{
    fmt::Display,
    ops::{BitAnd, BitOr},
};

use thiserror::Error;

use crate::{bool::cv::CharVec, ltl::cm::CharMatrix};

use super::traits::Commutativity;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
/// Binary LTL Operators: Or, And, Until, Release, Implication, and Equivalence
pub enum LtlBinaryOp {
    Or,
    And,
    Until,
    Release,
    Implies,
    Equivalent,
}

impl LtlBinaryOp {
    /// Returns a list of all binary operators.
    pub(crate) fn all() -> Vec<LtlBinaryOp> {
        use LtlBinaryOp::*;
        vec![Or, And, Until, Release, Implies, Equivalent]
    }

    /// Whether this LTL operator is boolean.
    pub(crate) fn is_boolean_monotone(&self) -> bool {
        matches!(
            self,
            LtlBinaryOp::Or | LtlBinaryOp::And | LtlBinaryOp::Equivalent
        )
    }

    /// Apply the operator to two characteristic vectors.
    ///
    /// # Panics
    ///
    /// Panics if the operator is not boolean.
    pub(crate) fn apply_cv(op: Self, lhs: CharVec, rhs: CharVec) -> CharVec {
        match op {
            LtlBinaryOp::Or => lhs.bitor(rhs),
            LtlBinaryOp::And => lhs.bitand(rhs),
            LtlBinaryOp::Implies => lhs.implies(rhs),
            LtlBinaryOp::Equivalent => lhs.equiv(rhs),
            _ => panic!(
                "Cannot apply non-boolean operator {} to characteristic vectors",
                op
            ),
        }
    }

    /// Apply the operator to two characteristic matrices.
    pub(crate) fn apply_cm(op: Self, lhs: &CharMatrix, rhs: &CharMatrix) -> CharMatrix {
        match op {
            LtlBinaryOp::Or => lhs.or(rhs),
            LtlBinaryOp::And => lhs.and(rhs),
            LtlBinaryOp::Until => lhs.until(rhs),
            LtlBinaryOp::Release => lhs.release(rhs),
            LtlBinaryOp::Implies => lhs.implies(rhs),
            LtlBinaryOp::Equivalent => lhs.equiv(rhs),
        }
    }
}

impl Commutativity for LtlBinaryOp {
    fn commutes(&self) -> bool {
        match self {
            LtlBinaryOp::Or | LtlBinaryOp::And | LtlBinaryOp::Equivalent => true,
            LtlBinaryOp::Until | LtlBinaryOp::Release | LtlBinaryOp::Implies => false,
        }
    }
}

#[derive(Debug, Error, PartialEq)]
#[error("Invalid binary operator '{}', expected one of '&', '|', 'U'.", .0)]
pub struct InvalidBinaryOp<'a>(&'a str);

impl<'a> TryFrom<&'a str> for LtlBinaryOp {
    type Error = InvalidBinaryOp<'a>;

    /// Parse a binary LTL operator from a string.
    ///
    /// This function parses strings as follows:
    ///
    /// | String |   Result  |
    /// |:-------|:---------------------|
    /// | `"\|"` | [`LtlBinaryOp::Or`]   |
    /// | `"&"`  | [`LtlBinaryOp::And`]  |
    /// | `"U"`  | [`LtlBinaryOp::Until`]|
    /// | `"R"`  | [`LtlBinaryOp::Release`]|
    /// | `"->"`  | [`LtlBinaryOp::Implies`]|
    /// | `"<->"`  | [`LtlBinaryOp::Equivalent`]|
    /// | Other value  | `Error`  |
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "|" => Ok(LtlBinaryOp::Or),
            "&" => Ok(LtlBinaryOp::And),
            "U" => Ok(LtlBinaryOp::Until),
            "R" => Ok(LtlBinaryOp::Release),
            "->" => Ok(LtlBinaryOp::Implies),
            "<->" => Ok(LtlBinaryOp::Equivalent),
            _ => Err(InvalidBinaryOp(value)),
        }
    }
}

impl Display for LtlBinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LtlBinaryOp::And => write!(f, "&"),
            LtlBinaryOp::Or => write!(f, "|"),
            LtlBinaryOp::Until => write!(f, "U"),
            LtlBinaryOp::Release => write!(f, "R"),
            LtlBinaryOp::Implies => write!(f, "->"),
            LtlBinaryOp::Equivalent => write!(f, "<->"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn string_try_into_binary_op() {
        let parsed = "|".try_into();
        assert_eq!(parsed, Ok(LtlBinaryOp::Or));

        let parsed = "&".try_into();
        assert_eq!(parsed, Ok(LtlBinaryOp::And));

        let parsed = "U".try_into();
        assert_eq!(parsed, Ok(LtlBinaryOp::Until));

        let parsed = "R".try_into();
        assert_eq!(parsed, Ok(LtlBinaryOp::Release));

        let parsed = "->".try_into();
        assert_eq!(parsed, Ok(LtlBinaryOp::Implies));

        let parsed = "<->".try_into();
        assert_eq!(parsed, Ok(LtlBinaryOp::Equivalent));

        let parsed: Result<LtlBinaryOp, _> = ":".try_into();
        assert!(parsed.is_err());
    }

    #[test]
    fn binary_op_display_then_parse_is_ident() {
        for op in LtlBinaryOp::all() {
            assert_eq!(Ok(op), format!("{op}").as_str().try_into())
        }
    }
}
