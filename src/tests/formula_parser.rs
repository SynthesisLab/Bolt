use crate::{
    formula::tree::parse_ltl_formula,
    tests::{G, Not},
};

use super::{AND, EQ, F, IMP, SX, U, X, build_atom};

#[test]
fn test_parsing_fixed() {
    let atomic_props: Vec<_> = ["a0", "a1"].into_iter().map(|s| s.to_owned()).collect();
    let a0 = || build_atom("a0", 0);
    let a1 = || build_atom("a1", 1);
    for (expr, expected) in [
        ("G(!(a0))", G(Not(a0()))),
        ("F(a0) -> (!(a0) U a1)", IMP(F(a0()), U(Not(a0()), a1()))),
        ("G(a1 -> G(!(a0)))", G(IMP(a1(), G(Not(a0()))))),
        ("F(a0)", F(a0())),
        ("G(a0)", G(a0())),
        ("G(!(a0))", G(Not(a0()))),
    ] {
        let f = parse_ltl_formula(expr, &atomic_props).unwrap();
        assert_eq!(f, expected)
    }
}

#[test]
fn test_parsing_gfand() {
    let atomic_props: Vec<_> = ["p1", "p2", "p3"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let p1 = || build_atom("p1", 0);
    let p2 = || build_atom("p2", 1);
    let p3 = || build_atom("p3", 2);
    for (expr, expected) in [("G p1 && F p2 && F p3", AND(AND(G(p1()), F(p2())), F(p3())))] {
        let f = parse_ltl_formula(expr, &atomic_props).unwrap();
        assert_eq!(f, expected)
    }
}

#[test]
fn test_double_unary() {
    let atomic_props: Vec<_> = ["a", "b", "c"].into_iter().map(|s| s.to_owned()).collect();

    let a = || build_atom("a", 0);
    let b = || build_atom("b", 1);
    let c = || build_atom("c", 2);
    for (expr, expected) in [
        ("X G a && F X[!] b", AND(X(G(a())), F(SX(b())))),
        (
            "X X a <=> G G b -> F F c",
            EQ(X(X(a())), IMP(G(G(b())), F(F(c())))),
        ),
    ] {
        let f = parse_ltl_formula(expr, &atomic_props).unwrap();
        assert_eq!(f, expected)
    }
}
