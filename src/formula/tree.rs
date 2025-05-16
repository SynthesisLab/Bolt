//! Explicit formula tree representation.
use std::{fmt::Display, rc::Rc};

use crate::{
    ltl::{AtomicProposition, cm::CharMatrix, trace::Trace},
    ops::{binary::LtlBinaryOp, unary::LtlUnaryOp},
};

/// Representation of an LTL as a tree of operators.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum FormulaTree {
    Atom(AtomicProposition),
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
            FormulaTree::Atom(AtomicProposition(_, i)) => {
                traces.iter().map(|t| t.atomic_propositions[*i]).collect()
            }
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
            FormulaTree::Atom(AtomicProposition(p, _)) => write!(f, "{p}"),
            FormulaTree::UnaryNode { op, child } => write!(f, "{op} ({child})"),
            FormulaTree::BinaryNode { op, left, right } => write!(f, "({left}) {op} ({right})"),
        }
    }
}

use pest::{Parser, iterators::Pairs, pratt_parser::PrattParser};
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "ltl.pest"]
struct LtlParser;

#[derive(Debug, Error, PartialEq)]
pub enum LtlParsingError {
    #[error("Parser (pest) Error")]
    PestError(#[from] pest::error::Error<Rule>),
    #[error("Parser returned empty `Pairs` object")]
    EmptyPairs,
    #[error("Atomic proposition {0} not found")]
    MissingAtomicProposition(String),
}

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            // Addition and subtract have equal precedence
            .op(Op::infix(implies, Right) | Op::infix(equiv, Right))
            .op(Op::infix(or, Left))
            .op(Op::infix(and, Left))
            .op(Op::infix(until, Right) | Op::infix(release, Right))
            .op(Op::prefix(globally) | Op::prefix(finally))
            .op(Op::prefix(strong_next) | Op::prefix(weak_next))
            .op(Op::prefix(not))
    };
}

pub fn parse_expr(expr: &str, atomic_props: &[String]) -> Result<FormulaTree, LtlParsingError> {
    let mut pairs = LtlParser::parse(Rule::ltl_expr, expr)?;
    let res = parse_pairs(
        pairs
            .next()
            .ok_or(LtlParsingError::EmptyPairs)?
            .into_inner(),
        atomic_props,
    )?;
    Ok(res)
}

fn parse_pairs(
    pairs: Pairs<Rule>,
    atomic_props: &[String],
) -> Result<FormulaTree, LtlParsingError> {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::prop => {
                let s = primary.as_str().to_owned();
                let i = atomic_props
                    .iter()
                    .enumerate()
                    .find_map(|(index, label)| if label == &s { Some(index) } else { None })
                    .ok_or_else(|| LtlParsingError::MissingAtomicProposition(s.clone()))?;
                Ok(FormulaTree::Atom(AtomicProposition(s, i)))
            }
            Rule::expr => parse_pairs(primary.into_inner(), atomic_props),
            rule => unreachable!(
                "FormulaTree::parse expected atom, found {:?} {}",
                rule,
                primary.as_str()
            ),
        })
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::or => LtlBinaryOp::Or,
                Rule::and => LtlBinaryOp::And,
                Rule::until => LtlBinaryOp::Until,
                Rule::release => LtlBinaryOp::Release,
                Rule::implies => LtlBinaryOp::Implies,
                Rule::equiv => LtlBinaryOp::Equivalent,
                rule => unreachable!(
                    "FormulaTree::parse expected binary operator, found {:?}",
                    rule
                ),
            };
            let f = FormulaTree::BinaryNode {
                op,
                left: Rc::from(lhs?),
                right: Rc::from(rhs?),
            };
            Ok(f)
        })
        .map_prefix(|op, child| {
            let op = match op.as_rule() {
                Rule::strong_next => LtlUnaryOp::StrongNext,
                Rule::weak_next => LtlUnaryOp::WeakNext,
                Rule::not => LtlUnaryOp::Not,
                Rule::globally => LtlUnaryOp::Globally,
                Rule::finally => LtlUnaryOp::Finally,
                rule => unreachable!(
                    "FormulaTree::parse expected unary operator, found {:?}",
                    rule
                ),
            };
            let f = FormulaTree::UnaryNode {
                op,
                child: Rc::from(child?),
            };
            Ok(f)
        })
        .parse(pairs)
}
