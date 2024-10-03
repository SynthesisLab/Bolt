//! Explicit formula tree representation.
use std::{fmt::Display, ops::Not, rc::Rc};

use crate::{
    ltl::{cm::CharMatrix, trace::Trace, Predicate, PredicateForm},
    ops::{binary::LtlBinaryOp, unary::LtlUnaryOp},
};

/// Representation of an LTL as a tree of operators.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum FormulaTree {
    Atom(Predicate),
    UnaryNode {
        op: LtlUnaryOp,
        child: Rc<FormulaTree>,
    },
    BinaryNode {
        op: LtlBinaryOp,
        left: Rc<FormulaTree>,
        right: Rc<FormulaTree>,
    },
}

impl FormulaTree {
    /// Compute the size of the formula.
    pub fn size(&self) -> usize {
        match self {
            FormulaTree::Atom(_) => 1,
            FormulaTree::UnaryNode { child, .. } => 1 + child.size(),
            FormulaTree::BinaryNode { left, right, .. } => 1 + left.size() + right.size(),
        }
    }

    /// Evaluate the formula on a set of input traces.
    pub fn eval(&self, traces: &[Trace]) -> CharMatrix {
        match self {
            FormulaTree::Atom(Predicate(_, pf)) => match *pf {
                PredicateForm::Positive(i) => traces.iter().map(|t| t.alphabet[i]).collect(),
                PredicateForm::Negative(i) => traces.iter().map(|t| t.alphabet[i].not()).collect(),
            },
            FormulaTree::UnaryNode { op, child } => {
                let cm = child.eval(traces);
                LtlUnaryOp::apply_cm(*op, &cm)
            }
            FormulaTree::BinaryNode { op, left, right } => {
                let cm_l = left.eval(traces);
                let cm_r = right.eval(traces);
                LtlBinaryOp::apply_cm(*op, &cm_l, &cm_r)
            }
        }
    }
}

impl Display for FormulaTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormulaTree::Atom(Predicate(p, _)) => write!(f, "{p}"),
            FormulaTree::UnaryNode { op, child } => write!(f, "{op} ({child})"),
            FormulaTree::BinaryNode { op, left, right } => write!(f, "({left}) {op} ({right})"),
        }
    }
}
