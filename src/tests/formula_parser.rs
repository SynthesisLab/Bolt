use crate::{
    formula::tree::parse_expr,
    tests::{G, Not},
};

use super::{AND, F, IMP, U, build_atom};

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
        let f = parse_expr(expr, &atomic_props).unwrap();
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
        let f = parse_expr(expr, &atomic_props).unwrap();
        assert_eq!(f, expected)
    }
}
