//! Generic formulas types.
pub mod tree;

use std::{fmt::Debug, rc::Rc};

use tree::FormulaTree;

use crate::{
    cache::FormulaCache,
    ops::{binary::LtlBinaryOp, unary::LtlUnaryOp},
};

use super::{
    ops::traits::{BinaryOp, UnaryOp},
    traits::{EqTarget, Hashed},
};

/// Generic abstraction for formulas.
///
/// The `Char` type parameter is the characteristic representation of the formula:
/// characteristic matrix for LTL formula, characteristic vectors for Boolean formulas.
///
/// `Char` is required to implement the [`Hashed`] trait, i.e. it can be converted to a hash.
/// The `node` attribute uses this hash to implicitly store the tree structure of the formula.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Formula<Char>
where
    Char: Hashed,
{
    pub(crate) charac: Char,
    pub(crate) size: usize,
    pub(crate) node: FormulaNode<Char>,
}

impl<Char> Formula<Char>
where
    Char: Hashed,
{
    pub(crate) fn new_base(char: Char, size: usize, base: Rc<FormulaTree>) -> Self {
        Self {
            charac: char,
            size,
            node: FormulaNode::Base(base),
        }
    }
}

impl<Char> Hashed for Formula<Char>
where
    Char: Hashed,
{
    type HashType = Char::HashType;

    fn hashed(&self) -> Self::HashType {
        self.charac.hashed()
    }
}

impl<Char> EqTarget for Formula<Char>
where
    Char: Hashed + EqTarget,
{
    type TargetType = Char::TargetType;

    fn eq_target(&self, target: &Self::TargetType) -> bool {
        self.charac.eq_target(target)
    }
}

pub(crate) fn apply_unary<Char>(op: LtlUnaryOp, f: &Formula<Char>) -> Formula<Char>
where
    Char: UnaryOp + Hashed,
{
    let charac = <Char as UnaryOp>::apply(op, &f.charac);
    let node: FormulaNode<_> = FormulaNode::Unary {
        op,
        child: f.charac.hashed(),
    };
    Formula {
        charac,
        size: f.size + 1,
        node,
    }
}

pub(crate) fn apply_binary<Char>(
    op: LtlBinaryOp,
    f1: &Formula<Char>,
    f2: &Formula<Char>,
) -> Formula<Char>
where
    Char: BinaryOp + Hashed,
{
    let charac = <Char as BinaryOp>::apply(op, &f1.charac, &f2.charac);
    let node: FormulaNode<_> = FormulaNode::Binary {
        op,
        left: f1.charac.hashed(),
        right: f2.charac.hashed(),
    };
    Formula {
        charac,
        size: f1.size + 1 + f2.size,
        node,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum FormulaNode<Char>
where
    Char: Hashed,
{
    Base(Rc<FormulaTree>),
    Unary {
        op: LtlUnaryOp,
        child: Char::HashType,
    },
    Binary {
        op: LtlBinaryOp,
        left: Char::HashType,
        right: Char::HashType,
    },
}

pub(crate) fn rebuild_formula<Char>(
    f: &Formula<Char>,
    cache: &impl FormulaCache<Char>,
) -> FormulaTree
where
    Char: Hashed,
{
    Rc::<FormulaTree>::unwrap_or_clone(rebuild_formula_aux(f, cache))
}

pub(crate) fn rebuild_formula_aux<Char>(
    f: &Formula<Char>,
    cache: &impl FormulaCache<Char>,
) -> Rc<FormulaTree>
where
    Char: Hashed,
{
    match &f.node {
        FormulaNode::Base(b) => b.clone(),
        FormulaNode::Unary { op, child } => Rc::from(FormulaTree::UnaryNode {
            op: *op,
            child: rebuild_formula_aux(cache.get(child).unwrap(), cache),
        }),
        FormulaNode::Binary { op, left, right } => Rc::from(FormulaTree::BinaryNode {
            op: *op,
            left: rebuild_formula_aux(cache.get(left).unwrap(), cache),
            right: rebuild_formula_aux(cache.get(right).unwrap(), cache),
        }),
    }
}
