use std::rc::Rc;

use crate::{
    algos::{atoms, create_initial_cache, enumeration::aux::enum_aux},
    formula::{rebuild_formula, tree::FormulaTree},
    ltl::{AtomicProposition, trace::parse_traces},
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

helper_ops_unary!(Globally as G, Finally as F, WeakNext as X, StrongNext as SX);
helper_ops_binary!(Until as U, Release as R, Or as OR, And as AND);
fn build_atom(s: &str, i: usize) -> FormulaTree {
    FormulaTree::Atom(AtomicProposition(s.into(), i))
}

fn test_ltl_search(instance: &str, expected: FormulaTree) {
    let instance = parse_traces(&instance);
    let v = expected.eval(&instance.traces).accepted_vec();
    assert_eq!(
        instance.target, v,
        "Expected formula does not satisfy input!"
    );

    let atoms = atoms(&instance.traces, instance.atomic_propositions);
    // Add initial formulas
    let (atom, mut ltl_cache) = create_initial_cache(atoms, &instance.target);
    if let Some(f) = atom {
        let f = rebuild_formula(&f, &ltl_cache);
        assert_eq!(f, expected);
        return;
    }

    let max_size = expected.size();

    // Ltl search
    let ltl_res = enum_aux(
        &mut ltl_cache,
        &instance.operators,
        &instance.target,
        max_size,
    );

    let f = rebuild_formula(&ltl_res.unwrap(), &ltl_cache);
    assert!(f.size() <= expected.size());
}

#[test]
fn globally_a() {
    let example = r#"{
    "positive_traces": [
        {
            "a": ["1", "1", "1", "1"]
        }
    ],
    "negative_traces": [
        {
            "a": ["1", "1", "1", "0"]
        },
        {
            "a": ["0", "1"]
        },
        {
            "a": ["0", "0", "1", "1"]
        },
        {
            "a": ["1", "0", "1", "0"]
        }
    ],
    "atomic_propositions": ["a"],
    "number_atomic_propositions": 1,
    "number_traces": 5,
    "number_positive_traces": 1,
    "number_negative_traces": 4,
    "max_length_traces": 4
}"#;

    let exp = G(build_atom("a", 0));
    test_ltl_search(example, exp);
}

#[test]
fn globally_a_or_b() {
    let example = r#"{
    "positive_traces": [
        {
            "a": ["0", "1", "1", "1"],
            "b": ["1", "0", "0", "0"]
        }
    ],
    "negative_traces": [
        {
            "a": ["1", "1", "1", "0"],
            "b": ["0", "0", "0", "0"]
        },
        {
            "a": ["0", "1"],
            "b": ["0", "0"]
        },
        {
            "a": ["0", "0", "1", "1"],
            "b": ["1", "0", "0", "0"]
        },
        {
            "b": ["1", "1", "0", "0", "0"],
            "a": ["1", "0", "1", "0", "1"]
        }
    ],
    "atomic_propositions": ["a", "b"],
    "number_atomic_propositions": 2,
    "number_traces": 5,
    "number_positive_traces": 1,
    "number_negative_traces": 4,
    "max_length_traces": 5
}"#;

    let exp = G(OR(build_atom("a", 0), build_atom("b", 1)));
    test_ltl_search(example, exp);
}
