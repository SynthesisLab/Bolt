mod end_to_end;
mod formula_parser;
use std::rc::Rc;

use crate::{
    formula::tree::FormulaTree,
    ltl::{AtomicProposition, Constant},
    ops::{binary::LtlBinaryOp, unary::LtlUnaryOp},
};

/// Helper macro to implement helper functions :-)
macro_rules! helper_ops_unary {
    ($( $f:ident as $g:ident ),*) => {
        $(
            #[allow(non_snake_case, dead_code)]
            pub(crate) fn $g(f: FormulaTree) -> FormulaTree {
                FormulaTree::UnaryNode {
                    op: LtlUnaryOp::$f,
                    child: Rc::from(f),
                }
            }
        )*
    };
}

macro_rules! helper_ops_binary {
    ($( $f:ident as $g:ident ),*) => {
        $(
            #[allow(non_snake_case, dead_code)]
            pub(crate) fn $g(left: FormulaTree, right: FormulaTree) -> FormulaTree {
                FormulaTree::BinaryNode {
                    op: LtlBinaryOp::$f,
                    left: Rc::from(left),
                    right: Rc::from(right),
                }
            }
        )*
    };
}

helper_ops_unary!(
    Not as Not,
    Globally as G,
    Finally as F,
    WeakNext as X,
    StrongNext as SX
);
helper_ops_binary!(
    Until as U,
    Release as R,
    Or as OR,
    And as AND,
    Implies as IMP,
    Equivalent as EQ
);

fn build_atom(s: &str, i: usize) -> FormulaTree {
    FormulaTree::Atom(AtomicProposition(s.into(), i))
}

fn build_const(c: Constant) -> FormulaTree {
    FormulaTree::Const(c)
}
