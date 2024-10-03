//! Unary LTL Operators
use std::fmt::Display;

use thiserror::Error;

use crate::ltl::cm::CharMatrix;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LtlUnaryOp {
    // Not,
    Next,
    Finally,
    Globally,
}

impl LtlUnaryOp {
    pub(crate) fn all() -> Vec<LtlUnaryOp> {
        use LtlUnaryOp::*;
        vec![Next, Finally, Globally]
        // vec![Not, Next, Finally, Globally]
    }

    pub(crate) fn is_boolean(&self) -> bool {
        match self {
            // LtlUnaryOp::Not => true,
            LtlUnaryOp::Next | LtlUnaryOp::Finally | LtlUnaryOp::Globally => false,
        }
    }

    pub(crate) fn apply_cm(op: Self, cm: &CharMatrix) -> CharMatrix {
        match op {
            // LtlUnaryOp::Not => cm.not(),
            LtlUnaryOp::Next => cm.next(),
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
    /// | `"X"`  | [`LtlUnaryOp::Next`]  |
    /// | `"F"`  | [`LtlUnaryOp::Finally`]   |
    /// | `"G"`  | [`LtlUnaryOp::Globally`]|
    /// | Other value  | `Error` |
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "X" => Ok(LtlUnaryOp::Next),
            "F" => Ok(LtlUnaryOp::Finally),
            "G" => Ok(LtlUnaryOp::Globally),
            _ => Err(InvalidUnaryOp(value)),
        }
    }
}

impl Display for LtlUnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LtlUnaryOp::Next => write!(f, "X"),
            LtlUnaryOp::Finally => write!(f, "F"),
            LtlUnaryOp::Globally => write!(f, "G"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn string_try_into_binary_op() {
        let parsed = "X".try_into();
        assert_eq!(parsed, Ok(LtlUnaryOp::Next));

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
