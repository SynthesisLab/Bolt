use std::rc::Rc;

use itertools::Itertools;

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

helper_ops_unary!(
    Not as Not,
    Globally as G,
    Finally as F,
    WeakNext as X,
    StrongNext as SX
);
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

/// Converts an array of array of array of ints (yes)
/// to a json string that looks like this:
/// ```json
/// [
///     {
///         "a": ["1", "1", "1", "0"],
///         "b": ["0", "0", "0", "0"]
///     },
///     {
///         "a": ["0", "1"],
///         "b": ["0", "0"]
///     },
///     {
///         "a": ["0", "0", "1", "1"],
///         "b": ["1", "0", "0", "0"]
///     },
/// ]
/// ```
/// The names ("a", "b" above) are given in the atomic_props array.
fn to_traces_str(traces: &[&[&[i32]]], atomic_props: &[&str]) -> String {
    format!(
        "[{}]",
        traces
            .iter()
            .map(|ts| {
                format!(
                    "{{ {} }}",
                    ts.iter()
                        .zip(atomic_props)
                        .map(|(t, a)| {
                            format!(
                                r#""{a}": [{}]"#,
                                t.iter().map(|s| format!(r#""{s}""#)).join(", ")
                            )
                        })
                        .join(", ")
                )
            })
            .join(", ")
    )
}

fn instance_str(
    positive_traces: &[&[&[i32]]],
    negative_traces: &[&[&[i32]]],
    atomic_props: &[&str],
) -> String {
    let positive_traces_str: String = to_traces_str(positive_traces, atomic_props);
    let negative_traces_str: String = to_traces_str(negative_traces, atomic_props);

    format!(
        r#"{{
        "positive_traces": {},
        "negative_traces": {},
        "atomic_propositions": {:?},
        "number_atomic_propositions": {},
        "number_traces": {},
        "number_positive_traces": {},
        "number_negative_traces": {},
        "max_length_traces": {}
    }}"#,
        positive_traces_str,
        negative_traces_str,
        atomic_props,
        atomic_props.len(),
        positive_traces.len() + negative_traces.len(),
        positive_traces.len(),
        negative_traces.len(),
        positive_traces
            .iter()
            .filter_map(|s| s.iter().map(|s2| s2.len()).max())
            .chain(
                negative_traces
                    .iter()
                    .filter_map(|s| s.iter().map(|s2| s2.len()).max())
            )
            .max()
            .unwrap()
    )
}

macro_rules! make_instance {
    (pos : [$( { $( $name:ident : [$($v:literal),*]),* }), *],
     neg: [$( { $( $name2:ident : [$($v2:literal),*]),* }), *]) => {
        &instance_str(
            &[ $( &[ $( &[ $( $v),* ]),* ]),*],
            &[ $( &[ $( &[ $( $v2),* ]),* ]),*],
            & helper!{ $([ $( $name ),* ]),* }
        )
    };
}

macro_rules! helper {
    ([ $( $values:ident ),*] $(, [ $( $v:ident ),*] )*) => { [$( stringify!($values) ),*]};
}

#[test]
fn strong_next_not_a() {
    let example = make_instance!(pos: [
        {
            a: [1, 0, 1, 1]
        },
        {
            a: [0, 0, 1]
        },
        {
            a: [1, 0, 1, 0]
        }
    ],
    neg: [
        {
            a: [1, 1, 1, 0]
        },
        {
            a: [0]
        },
        {
            a: [1]
        },
        {
            a: [0, 1, 1, 0, 1]
        },
        {
            a: [1, 1, 1, 1]
        }
    ]);

    let exp = SX(Not(build_atom("a", 0)));
    test_ltl_search(example, exp);
}

#[test]
fn globally_a() {
    let example = make_instance!(pos: [
        {
            a: [1, 1, 1, 1]
        }
    ],
    neg: [
        {
            a: [1, 1, 1, 0]
        },
        {
            a: [0, 1]
        },
        {
            a: [0, 0, 1, 1]
        },
        {
            a: [1, 0, 1, 0]
        }
    ]);

    let exp = G(build_atom("a", 0));
    test_ltl_search(example, exp);
}

#[test]
fn globally_a_or_b() {
    let example = make_instance!(
    pos: [
        {
            a: [0, 1, 1, 1],
            b: [1, 0, 0, 0]
        }
    ],
    neg: [
        {
            a: [1, 1, 1, 0],
            b: [0, 0, 0, 0]
        },
        {
            a: [0, 1],
            b: [0, 0]
        },
        {
            a: [0, 0, 1, 1],
            b: [1, 0, 0, 0]
        },
        {
            b: [1, 1, 0, 0, 0],
            a: [1, 0, 1, 0, 1]
        }
    ]);
    let exp = G(OR(build_atom("a", 0), build_atom("b", 1)));
    test_ltl_search(example, exp);
}

#[test]
fn finally_a_and_b() {
    let example = make_instance!(
    pos: [
        {
            a: [0, 0, 1, 1],
            b: [1, 0, 1, 0]
        },
        {
            a: [1, 0, 1, 1],
            b: [0, 1, 0, 1]
        },
        {
            a: [1, 1, 1],
            b: [0, 1, 1]
        }
    ],
    neg: [
        {
            a: [1, 0, 1, 0],
            b: [0, 1, 0, 1]
        },
        {
            a: [0, 1],
            b: [1, 0]
        },
        {
            a: [0, 0, 1, 1],
            b: [1, 0, 0, 0]
        },
        {
            b: [1, 1, 0, 0, 0],
            a: [0, 0, 1, 0, 1]
        }
    ]);
    let exp = F(AND(build_atom("a", 0), build_atom("b", 1)));
    test_ltl_search(example, exp);
}
