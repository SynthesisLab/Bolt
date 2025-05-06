//! Unary LTL Operators
use std::fmt::Display;

use thiserror::Error;

use crate::ltl::cm::CharMatrix;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LtlUnaryOp {
    // Not,
    WeakNext,
    StrongNext,
    Finally,
    Globally,
}

impl LtlUnaryOp {
    pub(crate) fn all() -> Vec<LtlUnaryOp> {
        use LtlUnaryOp::*;
        vec![WeakNext, StrongNext, Finally, Globally]
        // vec![Not, WeakNext, StrongNext, Finally, Globally]
    }

    pub(crate) fn is_boolean_monotone(&self) -> bool {
        match self {
            // LtlUnaryOp::Not => true,
            LtlUnaryOp::WeakNext
            | LtlUnaryOp::StrongNext
            | LtlUnaryOp::Finally
            | LtlUnaryOp::Globally => false,
        }
    }

    pub(crate) fn apply_cm(op: Self, cm: &CharMatrix) -> CharMatrix {
        match op {
            // LtlUnaryOp::Not => cm.not(),
            LtlUnaryOp::WeakNext => cm.weak_next(),
            LtlUnaryOp::StrongNext => cm.strong_next(),
            LtlUnaryOp::Finally => cm.finally(),
            LtlUnaryOp::Globally => cm.globally(),
        }
    }
}

#[derive(Debug, Error, PartialEq)]
#[error("Invalid unary operator '{}', expected one of 'X', 'F', 'G'.", .0)]
pub struct InvalidUnaryOp<'a>(&'a str);

impl<'a> TryFrom<&'a str> for LtlUnaryOp {
    type Error = InvalidUnaryOp<'a>;

    /// Parse an unary LTL operator from a string.
    ///
    /// This function parses strings as follows:
    ///
    /// | String |   Result  |
    /// |:-------|:---------------------|
    /// | `"X"`  | [`LtlUnaryOp::WeakNext`]  |
    /// | `"X[!]"`  | [`LtlUnaryOp::StrongNext`]  |
    /// | `"F"`  | [`LtlUnaryOp::Finally`]   |
    /// | `"G"`  | [`LtlUnaryOp::Globally`]|
    /// | Other value  | `Error` |
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "X" => Ok(LtlUnaryOp::WeakNext),
            "X[!]" => Ok(LtlUnaryOp::StrongNext),
            "F" => Ok(LtlUnaryOp::Finally),
            "G" => Ok(LtlUnaryOp::Globally),
            _ => Err(InvalidUnaryOp(value)),
        }
    }
}

impl Display for LtlUnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LtlUnaryOp::WeakNext => write!(f, "X"),
            LtlUnaryOp::StrongNext => write!(f, "X[!]"),
            LtlUnaryOp::Finally => write!(f, "F"),
            LtlUnaryOp::Globally => write!(f, "G"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn string_try_into_unary_op() {
        let parsed = "X".try_into();
        assert_eq!(parsed, Ok(LtlUnaryOp::WeakNext));

        let parsed = "X[!]".try_into();
        assert_eq!(parsed, Ok(LtlUnaryOp::StrongNext));

        let parsed = "F".try_into();
        assert_eq!(parsed, Ok(LtlUnaryOp::Finally));

        let parsed = "G".try_into();
        assert_eq!(parsed, Ok(LtlUnaryOp::Globally));

        let parsed: Result<LtlUnaryOp, _> = ":".try_into();
        assert!(parsed.is_err());
    }

    #[test]
    fn binary_op_display_then_parse_is_ident() {
        for op in LtlUnaryOp::all() {
            assert_eq!(Ok(op), format!("{op}").as_str().try_into())
        }
    }
}
