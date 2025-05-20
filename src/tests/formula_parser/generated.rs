use crate::formula::tree::parse_ltl_formula;

#[test]
fn test_operator_precedence_0() {
    let atomic_props: Vec<_> = [
        "init_counter_0",
        "inc_env",
        "counter_env_0",
        "counter_sys_0",
        "carry_env_0",
        "carry_sys_0",
        "inc_sys",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((! counter_sys_0) && (! inc_sys) && ((X[!] counter_env_0) -> init_counter_0) && (init_counter_0 -> (X counter_env_0)) && ((G (inc_env -> (! (X[!] inc_env)))) -> ((X[!] (G ((carry_sys_0 <-> inc_sys) && ((X[!] carry_env_0) -> inc_env) && (inc_env -> (X carry_env_0)) && ((X[!] counter_env_0) -> (! (counter_env_0 <-> (X[!] carry_env_0)))) && ((! (counter_env_0 <-> (X carry_env_0))) -> (X counter_env_0)) && ((X[!] counter_sys_0) -> (! (counter_sys_0 <-> (X[!] carry_sys_0)))) && ((! (counter_sys_0 <-> (X carry_sys_0))) -> (X counter_sys_0))))) && (X[!] (F (counter_env_0 <-> counter_sys_0))))))";
    let f_no_par = "! counter_sys_0 && ! inc_sys && (X[!] counter_env_0 -> init_counter_0) && (init_counter_0 -> X counter_env_0) && (G (inc_env -> ! X[!] inc_env) -> (X[!] G ((carry_sys_0 <-> inc_sys) && (X[!] carry_env_0 -> inc_env) && (inc_env -> X carry_env_0) && (X[!] counter_env_0 -> ! (counter_env_0 <-> X[!] carry_env_0)) && (! (counter_env_0 <-> X carry_env_0) -> X counter_env_0) && (X[!] counter_sys_0 -> ! (counter_sys_0 <-> X[!] carry_sys_0)) && (! (counter_sys_0 <-> X carry_sys_0) -> X counter_sys_0)) && X[!] F (counter_env_0 <-> counter_sys_0)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_1() {
    let atomic_props: Vec<_> = [
        "select_env_0",
        "change_env_0",
        "change_env_1",
        "change_env_2",
        "change_env_3",
        "change_env_4",
        "select_sys_0",
        "change_sys_0",
        "change_sys_1",
        "change_sys_2",
        "change_sys_3",
        "change_sys_4",
        "turn_sys",
        "turn_env",
        "heap_0_0",
        "heap_0_1",
        "heap_0_2",
        "heap_0_3",
        "heap_0_4",
        "heap_0_5",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((! turn_env) && turn_sys && ((! select_sys_0) -> heap_0_5) && (select_sys_0 -> (change_sys_0 || change_sys_1 || change_sys_2 || change_sys_3 || change_sys_4)) && ((G ((turn_env -> select_env_0) && (change_env_1 -> (! change_env_0)) && (change_env_2 -> (! change_env_0)) && (change_env_2 -> (! change_env_1)) && (change_env_3 -> (! change_env_0)) && (change_env_3 -> (! change_env_1)) && (change_env_3 -> (! change_env_2)) && (change_env_4 -> (! change_env_0)) && (change_env_4 -> (! change_env_1)) && (change_env_4 -> (! change_env_2)) && (change_env_4 -> (! change_env_3)) && (! (heap_0_0 && (! heap_0_0) && (X[!] select_env_0))) && (((! heap_0_0) && heap_0_1 && (X[!] select_env_0)) -> (X[!] change_env_0)) && (((! heap_0_0) && heap_0_2 && (X[!] select_env_0)) -> ((X[!] change_env_0) || (X[!] change_env_1))) && (((! heap_0_0) && heap_0_3 && (X[!] select_env_0)) -> ((X[!] change_env_0) || (X[!] change_env_1) || (X[!] change_env_2))) && (((! heap_0_0) && heap_0_4 && (X[!] select_env_0)) -> ((X[!] change_env_0) || (X[!] change_env_1) || (X[!] change_env_2) || (X[!] change_env_3))) && (((! heap_0_0) && heap_0_5 && (X[!] select_env_0)) -> ((X[!] change_env_0) || (X[!] change_env_1) || (X[!] change_env_2) || (X[!] change_env_3) || (X[!] change_env_4))))) -> ((G ((turn_sys -> select_sys_0) && (change_sys_1 -> (! change_sys_0)) && (change_sys_2 -> (! change_sys_0)) && (change_sys_2 -> (! change_sys_1)) && (change_sys_3 -> (! change_sys_0)) && (change_sys_3 -> (! change_sys_1)) && (change_sys_3 -> (! change_sys_2)) && (change_sys_4 -> (! change_sys_0)) && (change_sys_4 -> (! change_sys_1)) && (change_sys_4 -> (! change_sys_2)) && (change_sys_4 -> (! change_sys_3)) && ((! turn_env) <-> turn_sys) && (heap_0_1 -> (! heap_0_0)) && (heap_0_2 -> (! heap_0_0)) && (heap_0_2 -> (! heap_0_1)) && (heap_0_3 -> (! heap_0_0)) && (heap_0_3 -> (! heap_0_1)) && (heap_0_3 -> (! heap_0_2)) && (heap_0_4 -> (! heap_0_0)) && (heap_0_4 -> (! heap_0_1)) && (heap_0_4 -> (! heap_0_2)) && (heap_0_4 -> (! heap_0_3)) && (heap_0_5 -> (! heap_0_0)) && (heap_0_5 -> (! heap_0_1)) && (heap_0_5 -> (! heap_0_2)) && (heap_0_5 -> (! heap_0_3)) && (heap_0_5 -> (! heap_0_4)) && ((change_env_0 && select_env_0 && turn_env) -> heap_0_0) && ((change_env_1 && select_env_0 && turn_env) -> heap_0_1) && ((change_env_2 && select_env_0 && turn_env) -> heap_0_2) && ((change_env_3 && select_env_0 && turn_env) -> heap_0_3) && ((change_env_4 && select_env_0 && turn_env) -> heap_0_4) && ((change_sys_0 && select_sys_0 && turn_sys) -> heap_0_0) && ((change_sys_1 && select_sys_0 && turn_sys) -> heap_0_1) && ((change_sys_2 && select_sys_0 && turn_sys) -> heap_0_2) && ((change_sys_3 && select_sys_0 && turn_sys) -> heap_0_3) && ((change_sys_4 && select_sys_0 && turn_sys) -> heap_0_4) && (! (heap_0_0 && (! heap_0_0) && (X[!] select_sys_0))) && (((! heap_0_0) && heap_0_1 && (X[!] select_sys_0)) -> (X[!] change_sys_0)) && (((! heap_0_0) && heap_0_2 && (X[!] select_sys_0)) -> ((X[!] change_sys_0) || (X[!] change_sys_1))) && (((! heap_0_0) && heap_0_3 && (X[!] select_sys_0)) -> ((X[!] change_sys_0) || (X[!] change_sys_1) || (X[!] change_sys_2))) && (((! heap_0_0) && heap_0_4 && (X[!] select_sys_0)) -> ((X[!] change_sys_0) || (X[!] change_sys_1) || (X[!] change_sys_2) || (X[!] change_sys_3))) && (((! heap_0_0) && heap_0_5 && (X[!] select_sys_0)) -> ((X[!] change_sys_0) || (X[!] change_sys_1) || (X[!] change_sys_2) || (X[!] change_sys_3) || (X[!] change_sys_4))) && ((X[!] turn_sys) -> turn_env) && ((X[!] turn_env) -> turn_sys) && ((heap_0_0 && (X[!] turn_env) && (X[!] (! select_env_0))) -> (X[!] heap_0_0)) && ((heap_0_1 && (X[!] turn_env) && (X[!] (! select_env_0))) -> (X[!] heap_0_1)) && ((heap_0_2 && (X[!] turn_env) && (X[!] (! select_env_0))) -> (X[!] heap_0_2)) && ((heap_0_3 && (X[!] turn_env) && (X[!] (! select_env_0))) -> (X[!] heap_0_3)) && ((heap_0_4 && (X[!] turn_env) && (X[!] (! select_env_0))) -> (X[!] heap_0_4)) && ((heap_0_5 && (X[!] turn_env) && (X[!] (! select_env_0))) -> (X[!] heap_0_5)) && ((heap_0_0 && (X[!] turn_sys) && (X[!] (! select_sys_0))) -> (X[!] heap_0_0)) && ((heap_0_1 && (X[!] turn_sys) && (X[!] (! select_sys_0))) -> (X[!] heap_0_1)) && ((heap_0_2 && (X[!] turn_sys) && (X[!] (! select_sys_0))) -> (X[!] heap_0_2)) && ((heap_0_3 && (X[!] turn_sys) && (X[!] (! select_sys_0))) -> (X[!] heap_0_3)) && ((heap_0_4 && (X[!] turn_sys) && (X[!] (! select_sys_0))) -> (X[!] heap_0_4)) && ((heap_0_5 && (X[!] turn_sys) && (X[!] (! select_sys_0))) -> (X[!] heap_0_5)))) && ((! heap_0_0) U (heap_0_0 && turn_env)))))";
    let f_no_par = "! turn_env && turn_sys && (! select_sys_0 -> heap_0_5) && (select_sys_0 -> (change_sys_0 || change_sys_1 || change_sys_2 || change_sys_3 || change_sys_4)) && (G ((turn_env -> select_env_0) && (change_env_1 -> ! change_env_0) && (change_env_2 -> ! change_env_0) && (change_env_2 -> ! change_env_1) && (change_env_3 -> ! change_env_0) && (change_env_3 -> ! change_env_1) && (change_env_3 -> ! change_env_2) && (change_env_4 -> ! change_env_0) && (change_env_4 -> ! change_env_1) && (change_env_4 -> ! change_env_2) && (change_env_4 -> ! change_env_3) && ! (heap_0_0 && ! heap_0_0 && X[!] select_env_0) && ((! heap_0_0 && heap_0_1 && X[!] select_env_0) -> X[!] change_env_0) && ((! heap_0_0 && heap_0_2 && X[!] select_env_0) -> (X[!] change_env_0 || X[!] change_env_1)) && ((! heap_0_0 && heap_0_3 && X[!] select_env_0) -> (X[!] change_env_0 || X[!] change_env_1 || X[!] change_env_2)) && ((! heap_0_0 && heap_0_4 && X[!] select_env_0) -> (X[!] change_env_0 || X[!] change_env_1 || X[!] change_env_2 || X[!] change_env_3)) && ((! heap_0_0 && heap_0_5 && X[!] select_env_0) -> (X[!] change_env_0 || X[!] change_env_1 || X[!] change_env_2 || X[!] change_env_3 || X[!] change_env_4))) -> (G ((turn_sys -> select_sys_0) && (change_sys_1 -> ! change_sys_0) && (change_sys_2 -> ! change_sys_0) && (change_sys_2 -> ! change_sys_1) && (change_sys_3 -> ! change_sys_0) && (change_sys_3 -> ! change_sys_1) && (change_sys_3 -> ! change_sys_2) && (change_sys_4 -> ! change_sys_0) && (change_sys_4 -> ! change_sys_1) && (change_sys_4 -> ! change_sys_2) && (change_sys_4 -> ! change_sys_3) && (! turn_env <-> turn_sys) && (heap_0_1 -> ! heap_0_0) && (heap_0_2 -> ! heap_0_0) && (heap_0_2 -> ! heap_0_1) && (heap_0_3 -> ! heap_0_0) && (heap_0_3 -> ! heap_0_1) && (heap_0_3 -> ! heap_0_2) && (heap_0_4 -> ! heap_0_0) && (heap_0_4 -> ! heap_0_1) && (heap_0_4 -> ! heap_0_2) && (heap_0_4 -> ! heap_0_3) && (heap_0_5 -> ! heap_0_0) && (heap_0_5 -> ! heap_0_1) && (heap_0_5 -> ! heap_0_2) && (heap_0_5 -> ! heap_0_3) && (heap_0_5 -> ! heap_0_4) && ((change_env_0 && select_env_0 && turn_env) -> heap_0_0) && ((change_env_1 && select_env_0 && turn_env) -> heap_0_1) && ((change_env_2 && select_env_0 && turn_env) -> heap_0_2) && ((change_env_3 && select_env_0 && turn_env) -> heap_0_3) && ((change_env_4 && select_env_0 && turn_env) -> heap_0_4) && ((change_sys_0 && select_sys_0 && turn_sys) -> heap_0_0) && ((change_sys_1 && select_sys_0 && turn_sys) -> heap_0_1) && ((change_sys_2 && select_sys_0 && turn_sys) -> heap_0_2) && ((change_sys_3 && select_sys_0 && turn_sys) -> heap_0_3) && ((change_sys_4 && select_sys_0 && turn_sys) -> heap_0_4) && ! (heap_0_0 && ! heap_0_0 && X[!] select_sys_0) && ((! heap_0_0 && heap_0_1 && X[!] select_sys_0) -> X[!] change_sys_0) && ((! heap_0_0 && heap_0_2 && X[!] select_sys_0) -> (X[!] change_sys_0 || X[!] change_sys_1)) && ((! heap_0_0 && heap_0_3 && X[!] select_sys_0) -> (X[!] change_sys_0 || X[!] change_sys_1 || X[!] change_sys_2)) && ((! heap_0_0 && heap_0_4 && X[!] select_sys_0) -> (X[!] change_sys_0 || X[!] change_sys_1 || X[!] change_sys_2 || X[!] change_sys_3)) && ((! heap_0_0 && heap_0_5 && X[!] select_sys_0) -> (X[!] change_sys_0 || X[!] change_sys_1 || X[!] change_sys_2 || X[!] change_sys_3 || X[!] change_sys_4)) && (X[!] turn_sys -> turn_env) && (X[!] turn_env -> turn_sys) && ((heap_0_0 && X[!] turn_env && X[!] ! select_env_0) -> X[!] heap_0_0) && ((heap_0_1 && X[!] turn_env && X[!] ! select_env_0) -> X[!] heap_0_1) && ((heap_0_2 && X[!] turn_env && X[!] ! select_env_0) -> X[!] heap_0_2) && ((heap_0_3 && X[!] turn_env && X[!] ! select_env_0) -> X[!] heap_0_3) && ((heap_0_4 && X[!] turn_env && X[!] ! select_env_0) -> X[!] heap_0_4) && ((heap_0_5 && X[!] turn_env && X[!] ! select_env_0) -> X[!] heap_0_5) && ((heap_0_0 && X[!] turn_sys && X[!] ! select_sys_0) -> X[!] heap_0_0) && ((heap_0_1 && X[!] turn_sys && X[!] ! select_sys_0) -> X[!] heap_0_1) && ((heap_0_2 && X[!] turn_sys && X[!] ! select_sys_0) -> X[!] heap_0_2) && ((heap_0_3 && X[!] turn_sys && X[!] ! select_sys_0) -> X[!] heap_0_3) && ((heap_0_4 && X[!] turn_sys && X[!] ! select_sys_0) -> X[!] heap_0_4) && ((heap_0_5 && X[!] turn_sys && X[!] ! select_sys_0) -> X[!] heap_0_5)) && (! heap_0_0 U (heap_0_0 && turn_env))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_2() {
    let atomic_props: Vec<_> = [
        "select_env_0",
        "select_env_1",
        "select_env_2",
        "change_env_0",
        "change_env_1",
        "change_env_2",
        "select_sys_0",
        "select_sys_1",
        "select_sys_2",
        "change_sys_0",
        "change_sys_1",
        "change_sys_2",
        "turn_sys",
        "turn_env",
        "heap_0_0",
        "heap_0_1",
        "heap_0_2",
        "heap_0_3",
        "heap_1_0",
        "heap_1_1",
        "heap_1_2",
        "heap_1_3",
        "heap_2_0",
        "heap_2_1",
        "heap_2_2",
        "heap_2_3",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((! turn_env) && turn_sys && ((! select_sys_0) -> heap_0_3) && ((! select_sys_1) -> heap_1_3) && ((! select_sys_2) -> heap_2_3) && (select_sys_0 -> (change_sys_0 || change_sys_1 || change_sys_2)) && (select_sys_1 -> (change_sys_0 || change_sys_1 || change_sys_2)) && (select_sys_2 -> (change_sys_0 || change_sys_1 || change_sys_2)) && ((G ((turn_env -> (select_env_0 || select_env_1 || select_env_2)) && (select_env_1 -> (! select_env_0)) && (select_env_2 -> (! select_env_0)) && (select_env_2 -> (! select_env_1)) && (change_env_1 -> (! change_env_0)) && (change_env_2 -> (! change_env_0)) && (change_env_2 -> (! change_env_1)) && (! (heap_0_0 && ((! heap_0_0) || (! heap_1_0) || (! heap_2_0)) && (X[!] select_env_0))) && ((heap_0_1 && ((! heap_0_0) || (! heap_1_0) || (! heap_2_0)) && (X[!] select_env_0)) -> (X[!] change_env_0)) && ((heap_0_2 && ((! heap_0_0) || (! heap_1_0) || (! heap_2_0)) && (X[!] select_env_0)) -> ((X[!] change_env_0) || (X[!] change_env_1))) && ((heap_0_3 && ((! heap_0_0) || (! heap_1_0) || (! heap_2_0)) && (X[!] select_env_0)) -> ((X[!] change_env_0) || (X[!] change_env_1) || (X[!] change_env_2))) && (! (heap_1_0 && ((! heap_0_0) || (! heap_1_0) || (! heap_2_0)) && (X[!] select_env_1))) && ((heap_1_1 && ((! heap_0_0) || (! heap_1_0) || (! heap_2_0)) && (X[!] select_env_1)) -> (X[!] change_env_0)) && ((heap_1_2 && ((! heap_0_0) || (! heap_1_0) || (! heap_2_0)) && (X[!] select_env_1)) -> ((X[!] change_env_0) || (X[!] change_env_1))) && ((heap_1_3 && ((! heap_0_0) || (! heap_1_0) || (! heap_2_0)) && (X[!] select_env_1)) -> ((X[!] change_env_0) || (X[!] change_env_1) || (X[!] change_env_2))) && (! (heap_2_0 && ((! heap_0_0) || (! heap_1_0) || (! heap_2_0)) && (X[!] select_env_2))) && ((heap_2_1 && ((! heap_0_0) || (! heap_1_0) || (! heap_2_0)) && (X[!] select_env_2)) -> (X[!] change_env_0)) && ((heap_2_2 && ((! heap_0_0) || (! heap_1_0) || (! heap_2_0)) && (X[!] select_env_2)) -> ((X[!] change_env_0) || (X[!] change_env_1))) && ((heap_2_3 && ((! heap_0_0) || (! heap_1_0) || (! heap_2_0)) && (X[!] select_env_2)) -> ((X[!] change_env_0) || (X[!] change_env_1) || (X[!] change_env_2))))) -> ((G ((turn_sys -> (select_sys_0 || select_sys_1 || select_sys_2)) && (select_sys_1 -> (! select_sys_0)) && (select_sys_2 -> (! select_sys_0)) && (select_sys_2 -> (! select_sys_1)) && (change_sys_1 -> (! change_sys_0)) && (change_sys_2 -> (! change_sys_0)) && (change_sys_2 -> (! change_sys_1)) && ((! turn_env) <-> turn_sys) && (heap_0_1 -> (! heap_0_0)) && (heap_0_2 -> (! heap_0_0)) && (heap_0_2 -> (! heap_0_1)) && (heap_0_3 -> (! heap_0_0)) && (heap_0_3 -> (! heap_0_1)) && (heap_0_3 -> (! heap_0_2)) && (heap_1_1 -> (! heap_1_0)) && (heap_1_2 -> (! heap_1_0)) && (heap_1_2 -> (! heap_1_1)) && (heap_1_3 -> (! heap_1_0)) && (heap_1_3 -> (! heap_1_1)) && (heap_1_3 -> (! heap_1_2)) && (heap_2_1 -> (! heap_2_0)) && (heap_2_2 -> (! heap_2_0)) && (heap_2_2 -> (! heap_2_1)) && (heap_2_3 -> (! heap_2_0)) && (heap_2_3 -> (! heap_2_1)) && (heap_2_3 -> (! heap_2_2)) && ((change_env_0 && select_env_0 && turn_env) -> heap_0_0) && ((change_env_1 && select_env_0 && turn_env) -> heap_0_1) && ((change_env_2 && select_env_0 && turn_env) -> heap_0_2) && ((change_env_0 && select_env_1 && turn_env) -> heap_1_0) && ((change_env_1 && select_env_1 && turn_env) -> heap_1_1) && ((change_env_2 && select_env_1 && turn_env) -> heap_1_2) && ((change_env_0 && select_env_2 && turn_env) -> heap_2_0) && ((change_env_1 && select_env_2 && turn_env) -> heap_2_1) && ((change_env_2 && select_env_2 && turn_env) -> heap_2_2) && ((change_sys_0 && select_sys_0 && turn_sys) -> heap_0_0) && ((change_sys_1 && select_sys_0 && turn_sys) -> heap_0_1) && ((change_sys_2 && select_sys_0 && turn_sys) -> heap_0_2) && ((change_sys_0 && select_sys_1 && turn_sys) -> heap_1_0) && ((change_sys_1 && select_sys_1 && turn_sys) -> heap_1_1) && ((change_sys_2 && select_sys_1 && turn_sys) -> heap_1_2) && ((change_sys_0 && select_sys_2 && turn_sys) -> heap_2_0) && ((change_sys_1 && select_sys_2 && turn_sys) -> heap_2_1) && ((change_sys_2 && select_sys_2 && turn_sys) -> heap_2_2) && (! (heap_0_0 && ((! heap_0_0) || (! heap_1_0) || (! heap_2_0)) && (X[!] select_sys_0))) && ((heap_0_1 && ((! heap_0_0) || (! heap_1_0) || (! heap_2_0)) && (X[!] select_sys_0)) -> (X[!] change_sys_0)) && ((heap_0_2 && ((! heap_0_0) || (! heap_1_0) || (! heap_2_0)) && (X[!] select_sys_0)) -> ((X[!] change_sys_0) || (X[!] change_sys_1))) && ((heap_0_3 && ((! heap_0_0) || (! heap_1_0) || (! heap_2_0)) && (X[!] select_sys_0)) -> ((X[!] change_sys_0) || (X[!] change_sys_1) || (X[!] change_sys_2))) && (! (heap_1_0 && ((! heap_0_0) || (! heap_1_0) || (! heap_2_0)) && (X[!] select_sys_1))) && ((heap_1_1 && ((! heap_0_0) || (! heap_1_0) || (! heap_2_0)) && (X[!] select_sys_1)) -> (X[!] change_sys_0)) && ((heap_1_2 && ((! heap_0_0) || (! heap_1_0) || (! heap_2_0)) && (X[!] select_sys_1)) -> ((X[!] change_sys_0) || (X[!] change_sys_1))) && ((heap_1_3 && ((! heap_0_0) || (! heap_1_0) || (! heap_2_0)) && (X[!] select_sys_1)) -> ((X[!] change_sys_0) || (X[!] change_sys_1) || (X[!] change_sys_2))) && (! (heap_2_0 && ((! heap_0_0) || (! heap_1_0) || (! heap_2_0)) && (X[!] select_sys_2))) && ((heap_2_1 && ((! heap_0_0) || (! heap_1_0) || (! heap_2_0)) && (X[!] select_sys_2)) -> (X[!] change_sys_0)) && ((heap_2_2 && ((! heap_0_0) || (! heap_1_0) || (! heap_2_0)) && (X[!] select_sys_2)) -> ((X[!] change_sys_0) || (X[!] change_sys_1))) && ((heap_2_3 && ((! heap_0_0) || (! heap_1_0) || (! heap_2_0)) && (X[!] select_sys_2)) -> ((X[!] change_sys_0) || (X[!] change_sys_1) || (X[!] change_sys_2))) && ((X[!] turn_sys) -> turn_env) && ((X[!] turn_env) -> turn_sys) && ((heap_0_0 && (X[!] turn_env) && (X[!] (! select_env_0))) -> (X[!] heap_0_0)) && ((heap_0_1 && (X[!] turn_env) && (X[!] (! select_env_0))) -> (X[!] heap_0_1)) && ((heap_0_2 && (X[!] turn_env) && (X[!] (! select_env_0))) -> (X[!] heap_0_2)) && ((heap_0_3 && (X[!] turn_env) && (X[!] (! select_env_0))) -> (X[!] heap_0_3)) && ((heap_1_0 && (X[!] turn_env) && (X[!] (! select_env_1))) -> (X[!] heap_1_0)) && ((heap_1_1 && (X[!] turn_env) && (X[!] (! select_env_1))) -> (X[!] heap_1_1)) && ((heap_1_2 && (X[!] turn_env) && (X[!] (! select_env_1))) -> (X[!] heap_1_2)) && ((heap_1_3 && (X[!] turn_env) && (X[!] (! select_env_1))) -> (X[!] heap_1_3)) && ((heap_2_0 && (X[!] turn_env) && (X[!] (! select_env_2))) -> (X[!] heap_2_0)) && ((heap_2_1 && (X[!] turn_env) && (X[!] (! select_env_2))) -> (X[!] heap_2_1)) && ((heap_2_2 && (X[!] turn_env) && (X[!] (! select_env_2))) -> (X[!] heap_2_2)) && ((heap_2_3 && (X[!] turn_env) && (X[!] (! select_env_2))) -> (X[!] heap_2_3)) && ((heap_0_0 && (X[!] turn_sys) && (X[!] (! select_sys_0))) -> (X[!] heap_0_0)) && ((heap_0_1 && (X[!] turn_sys) && (X[!] (! select_sys_0))) -> (X[!] heap_0_1)) && ((heap_0_2 && (X[!] turn_sys) && (X[!] (! select_sys_0))) -> (X[!] heap_0_2)) && ((heap_0_3 && (X[!] turn_sys) && (X[!] (! select_sys_0))) -> (X[!] heap_0_3)) && ((heap_1_0 && (X[!] turn_sys) && (X[!] (! select_sys_1))) -> (X[!] heap_1_0)) && ((heap_1_1 && (X[!] turn_sys) && (X[!] (! select_sys_1))) -> (X[!] heap_1_1)) && ((heap_1_2 && (X[!] turn_sys) && (X[!] (! select_sys_1))) -> (X[!] heap_1_2)) && ((heap_1_3 && (X[!] turn_sys) && (X[!] (! select_sys_1))) -> (X[!] heap_1_3)) && ((heap_2_0 && (X[!] turn_sys) && (X[!] (! select_sys_2))) -> (X[!] heap_2_0)) && ((heap_2_1 && (X[!] turn_sys) && (X[!] (! select_sys_2))) -> (X[!] heap_2_1)) && ((heap_2_2 && (X[!] turn_sys) && (X[!] (! select_sys_2))) -> (X[!] heap_2_2)) && ((heap_2_3 && (X[!] turn_sys) && (X[!] (! select_sys_2))) -> (X[!] heap_2_3)))) && (((! heap_0_0) || (! heap_1_0) || (! heap_2_0)) U (heap_0_0 && heap_1_0 && heap_2_0 && turn_env)))))";
    let f_no_par = "! turn_env && turn_sys && (! select_sys_0 -> heap_0_3) && (! select_sys_1 -> heap_1_3) && (! select_sys_2 -> heap_2_3) && (select_sys_0 -> (change_sys_0 || change_sys_1 || change_sys_2)) && (select_sys_1 -> (change_sys_0 || change_sys_1 || change_sys_2)) && (select_sys_2 -> (change_sys_0 || change_sys_1 || change_sys_2)) && (G ((turn_env -> (select_env_0 || select_env_1 || select_env_2)) && (select_env_1 -> ! select_env_0) && (select_env_2 -> ! select_env_0) && (select_env_2 -> ! select_env_1) && (change_env_1 -> ! change_env_0) && (change_env_2 -> ! change_env_0) && (change_env_2 -> ! change_env_1) && ! (heap_0_0 && (! heap_0_0 || ! heap_1_0 || ! heap_2_0) && X[!] select_env_0) && ((heap_0_1 && (! heap_0_0 || ! heap_1_0 || ! heap_2_0) && X[!] select_env_0) -> X[!] change_env_0) && ((heap_0_2 && (! heap_0_0 || ! heap_1_0 || ! heap_2_0) && X[!] select_env_0) -> (X[!] change_env_0 || X[!] change_env_1)) && ((heap_0_3 && (! heap_0_0 || ! heap_1_0 || ! heap_2_0) && X[!] select_env_0) -> (X[!] change_env_0 || X[!] change_env_1 || X[!] change_env_2)) && ! (heap_1_0 && (! heap_0_0 || ! heap_1_0 || ! heap_2_0) && X[!] select_env_1) && ((heap_1_1 && (! heap_0_0 || ! heap_1_0 || ! heap_2_0) && X[!] select_env_1) -> X[!] change_env_0) && ((heap_1_2 && (! heap_0_0 || ! heap_1_0 || ! heap_2_0) && X[!] select_env_1) -> (X[!] change_env_0 || X[!] change_env_1)) && ((heap_1_3 && (! heap_0_0 || ! heap_1_0 || ! heap_2_0) && X[!] select_env_1) -> (X[!] change_env_0 || X[!] change_env_1 || X[!] change_env_2)) && ! (heap_2_0 && (! heap_0_0 || ! heap_1_0 || ! heap_2_0) && X[!] select_env_2) && ((heap_2_1 && (! heap_0_0 || ! heap_1_0 || ! heap_2_0) && X[!] select_env_2) -> X[!] change_env_0) && ((heap_2_2 && (! heap_0_0 || ! heap_1_0 || ! heap_2_0) && X[!] select_env_2) -> (X[!] change_env_0 || X[!] change_env_1)) && ((heap_2_3 && (! heap_0_0 || ! heap_1_0 || ! heap_2_0) && X[!] select_env_2) -> (X[!] change_env_0 || X[!] change_env_1 || X[!] change_env_2))) -> (G ((turn_sys -> (select_sys_0 || select_sys_1 || select_sys_2)) && (select_sys_1 -> ! select_sys_0) && (select_sys_2 -> ! select_sys_0) && (select_sys_2 -> ! select_sys_1) && (change_sys_1 -> ! change_sys_0) && (change_sys_2 -> ! change_sys_0) && (change_sys_2 -> ! change_sys_1) && (! turn_env <-> turn_sys) && (heap_0_1 -> ! heap_0_0) && (heap_0_2 -> ! heap_0_0) && (heap_0_2 -> ! heap_0_1) && (heap_0_3 -> ! heap_0_0) && (heap_0_3 -> ! heap_0_1) && (heap_0_3 -> ! heap_0_2) && (heap_1_1 -> ! heap_1_0) && (heap_1_2 -> ! heap_1_0) && (heap_1_2 -> ! heap_1_1) && (heap_1_3 -> ! heap_1_0) && (heap_1_3 -> ! heap_1_1) && (heap_1_3 -> ! heap_1_2) && (heap_2_1 -> ! heap_2_0) && (heap_2_2 -> ! heap_2_0) && (heap_2_2 -> ! heap_2_1) && (heap_2_3 -> ! heap_2_0) && (heap_2_3 -> ! heap_2_1) && (heap_2_3 -> ! heap_2_2) && ((change_env_0 && select_env_0 && turn_env) -> heap_0_0) && ((change_env_1 && select_env_0 && turn_env) -> heap_0_1) && ((change_env_2 && select_env_0 && turn_env) -> heap_0_2) && ((change_env_0 && select_env_1 && turn_env) -> heap_1_0) && ((change_env_1 && select_env_1 && turn_env) -> heap_1_1) && ((change_env_2 && select_env_1 && turn_env) -> heap_1_2) && ((change_env_0 && select_env_2 && turn_env) -> heap_2_0) && ((change_env_1 && select_env_2 && turn_env) -> heap_2_1) && ((change_env_2 && select_env_2 && turn_env) -> heap_2_2) && ((change_sys_0 && select_sys_0 && turn_sys) -> heap_0_0) && ((change_sys_1 && select_sys_0 && turn_sys) -> heap_0_1) && ((change_sys_2 && select_sys_0 && turn_sys) -> heap_0_2) && ((change_sys_0 && select_sys_1 && turn_sys) -> heap_1_0) && ((change_sys_1 && select_sys_1 && turn_sys) -> heap_1_1) && ((change_sys_2 && select_sys_1 && turn_sys) -> heap_1_2) && ((change_sys_0 && select_sys_2 && turn_sys) -> heap_2_0) && ((change_sys_1 && select_sys_2 && turn_sys) -> heap_2_1) && ((change_sys_2 && select_sys_2 && turn_sys) -> heap_2_2) && ! (heap_0_0 && (! heap_0_0 || ! heap_1_0 || ! heap_2_0) && X[!] select_sys_0) && ((heap_0_1 && (! heap_0_0 || ! heap_1_0 || ! heap_2_0) && X[!] select_sys_0) -> X[!] change_sys_0) && ((heap_0_2 && (! heap_0_0 || ! heap_1_0 || ! heap_2_0) && X[!] select_sys_0) -> (X[!] change_sys_0 || X[!] change_sys_1)) && ((heap_0_3 && (! heap_0_0 || ! heap_1_0 || ! heap_2_0) && X[!] select_sys_0) -> (X[!] change_sys_0 || X[!] change_sys_1 || X[!] change_sys_2)) && ! (heap_1_0 && (! heap_0_0 || ! heap_1_0 || ! heap_2_0) && X[!] select_sys_1) && ((heap_1_1 && (! heap_0_0 || ! heap_1_0 || ! heap_2_0) && X[!] select_sys_1) -> X[!] change_sys_0) && ((heap_1_2 && (! heap_0_0 || ! heap_1_0 || ! heap_2_0) && X[!] select_sys_1) -> (X[!] change_sys_0 || X[!] change_sys_1)) && ((heap_1_3 && (! heap_0_0 || ! heap_1_0 || ! heap_2_0) && X[!] select_sys_1) -> (X[!] change_sys_0 || X[!] change_sys_1 || X[!] change_sys_2)) && ! (heap_2_0 && (! heap_0_0 || ! heap_1_0 || ! heap_2_0) && X[!] select_sys_2) && ((heap_2_1 && (! heap_0_0 || ! heap_1_0 || ! heap_2_0) && X[!] select_sys_2) -> X[!] change_sys_0) && ((heap_2_2 && (! heap_0_0 || ! heap_1_0 || ! heap_2_0) && X[!] select_sys_2) -> (X[!] change_sys_0 || X[!] change_sys_1)) && ((heap_2_3 && (! heap_0_0 || ! heap_1_0 || ! heap_2_0) && X[!] select_sys_2) -> (X[!] change_sys_0 || X[!] change_sys_1 || X[!] change_sys_2)) && (X[!] turn_sys -> turn_env) && (X[!] turn_env -> turn_sys) && ((heap_0_0 && X[!] turn_env && X[!] ! select_env_0) -> X[!] heap_0_0) && ((heap_0_1 && X[!] turn_env && X[!] ! select_env_0) -> X[!] heap_0_1) && ((heap_0_2 && X[!] turn_env && X[!] ! select_env_0) -> X[!] heap_0_2) && ((heap_0_3 && X[!] turn_env && X[!] ! select_env_0) -> X[!] heap_0_3) && ((heap_1_0 && X[!] turn_env && X[!] ! select_env_1) -> X[!] heap_1_0) && ((heap_1_1 && X[!] turn_env && X[!] ! select_env_1) -> X[!] heap_1_1) && ((heap_1_2 && X[!] turn_env && X[!] ! select_env_1) -> X[!] heap_1_2) && ((heap_1_3 && X[!] turn_env && X[!] ! select_env_1) -> X[!] heap_1_3) && ((heap_2_0 && X[!] turn_env && X[!] ! select_env_2) -> X[!] heap_2_0) && ((heap_2_1 && X[!] turn_env && X[!] ! select_env_2) -> X[!] heap_2_1) && ((heap_2_2 && X[!] turn_env && X[!] ! select_env_2) -> X[!] heap_2_2) && ((heap_2_3 && X[!] turn_env && X[!] ! select_env_2) -> X[!] heap_2_3) && ((heap_0_0 && X[!] turn_sys && X[!] ! select_sys_0) -> X[!] heap_0_0) && ((heap_0_1 && X[!] turn_sys && X[!] ! select_sys_0) -> X[!] heap_0_1) && ((heap_0_2 && X[!] turn_sys && X[!] ! select_sys_0) -> X[!] heap_0_2) && ((heap_0_3 && X[!] turn_sys && X[!] ! select_sys_0) -> X[!] heap_0_3) && ((heap_1_0 && X[!] turn_sys && X[!] ! select_sys_1) -> X[!] heap_1_0) && ((heap_1_1 && X[!] turn_sys && X[!] ! select_sys_1) -> X[!] heap_1_1) && ((heap_1_2 && X[!] turn_sys && X[!] ! select_sys_1) -> X[!] heap_1_2) && ((heap_1_3 && X[!] turn_sys && X[!] ! select_sys_1) -> X[!] heap_1_3) && ((heap_2_0 && X[!] turn_sys && X[!] ! select_sys_2) -> X[!] heap_2_0) && ((heap_2_1 && X[!] turn_sys && X[!] ! select_sys_2) -> X[!] heap_2_1) && ((heap_2_2 && X[!] turn_sys && X[!] ! select_sys_2) -> X[!] heap_2_2) && ((heap_2_3 && X[!] turn_sys && X[!] ! select_sys_2) -> X[!] heap_2_3)) && ((! heap_0_0 || ! heap_1_0 || ! heap_2_0) U (heap_0_0 && heap_1_0 && heap_2_0 && turn_env))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_3() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a0) -> (((F a0) && (a5 U a0) && (G (a8 && ((! a4) -> ((! a4) U ((! a4) && a5))))) && (F a6)) -> (((a1 U a6) && (G (a5 -> (G (! a7)))) && (G (! a2))) || ((F (a2 && (F a7))) && (G (a7 && ((! a1) -> ((! a1) U ((! a1) && a9))))) && (G (a1 -> (G a8))) && (G (! a6)) && (G (! a5))))))";
    let f_no_par = "F a0 -> ((F a0 && (a5 U a0) && G (a8 && (! a4 -> (! a4 U (! a4 && a5)))) && F a6) -> (((a1 U a6) && G (a5 -> G ! a7) && G ! a2) || (F (a2 && F a7) && G (a7 && (! a1 -> (! a1 U (! a1 && a9)))) && G (a1 -> G a8) && G ! a6 && G ! a5)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_4() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a0) -> (((F a0) && (a5 U a0) && (G (a8 && ((! a4) -> ((! a4) U ((! a4) && a10))))) && (F a1)) -> (((a5 U a1) && (G (a7 -> (G (! a6)))) && (G (! a5))) || ((G (! a6)) && (F (a5 && (F a2))) && (G (a7 && ((! a10) -> ((! a10) U (a1 && (! a10)))))) && (G (a8 -> (G a7))) && (G (! a9))))))";
    let f_no_par = "F a0 -> ((F a0 && (a5 U a0) && G (a8 && (! a4 -> (! a4 U (! a4 && a10)))) && F a1) -> (((a5 U a1) && G (a7 -> G ! a6) && G ! a5) || (G ! a6 && F (a5 && F a2) && G (a7 && (! a10 -> (! a10 U (a1 && ! a10)))) && G (a8 -> G a7) && G ! a9)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_5() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10", "a11",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a10) -> (((a5 U a10) && (F a0) && (G (a2 && ((! a8) -> ((! a8) U (a4 && (! a8)))))) && (F a5)) -> (((a8 U a5) && (G (a1 -> (G (! a11)))) && (G (! a6))) || ((F (a6 && (F a7))) && (G (a5 && ((! a2) -> ((! a2) U ((! a2) && a7))))) && (G (a1 -> (G a7))) && (G (! a7)) && (G (! a9))))))";
    let f_no_par = "F a10 -> (((a5 U a10) && F a0 && G (a2 && (! a8 -> (! a8 U (a4 && ! a8)))) && F a5) -> (((a8 U a5) && G (a1 -> G ! a11) && G ! a6) || (F (a6 && F a7) && G (a5 && (! a2 -> (! a2 U (! a2 && a7)))) && G (a1 -> G a7) && G ! a7 && G ! a9)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_6() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10", "a11", "a12",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a10) -> (((a5 U a10) && (F a0) && (G (a2 && ((! a8) -> ((! a8) U (a4 && (! a8)))))) && (F a5)) -> (((a8 U a5) && (G (a1 -> (G (! a11)))) && (G (! a6))) || ((G (! a11)) && (F (a6 && (F a11))) && (G (a7 && ((! a5) -> ((! a5) U (a2 && (! a5)))))) && (G (a12 -> (G a7))) && (G (! a1))))))";
    let f_no_par = "F a10 -> (((a5 U a10) && F a0 && G (a2 && (! a8 -> (! a8 U (a4 && ! a8)))) && F a5) -> (((a8 U a5) && G (a1 -> G ! a11) && G ! a6) || (G ! a11 && F (a6 && F a11) && G (a7 && (! a5 -> (! a5 U (a2 && ! a5)))) && G (a12 -> G a7) && G ! a1)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_7() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10", "a11", "a12", "a13",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a10) -> (((a5 U a10) && (F a13) && (G (a13 && ((! a0) -> ((! a0) U ((! a0) && a2))))) && (F a4)) -> (((a8 U a4) && (G (a5 -> (G (! a8)))) && (G (! a11))) || ((F (a11 && (F a12))) && (G (a1 && ((! a6) -> ((! a6) U ((! a6) && a11))))) && (G (a5 -> (G a7))) && (G (! a2)) && (G (! a7))))))";
    let f_no_par = "F a10 -> (((a5 U a10) && F a13 && G (a13 && (! a0 -> (! a0 U (! a0 && a2)))) && F a4) -> (((a8 U a4) && G (a5 -> G ! a8) && G ! a11) || (F (a11 && F a12) && G (a1 && (! a6 -> (! a6 U (! a6 && a11)))) && G (a5 -> G a7) && G ! a2 && G ! a7)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_8() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a0) -> (((F a0) && (a1 U a0) && (G (a2 && ((! a1) -> ((! a1) U (a0 && (! a1))))))) -> (((a2 U a0) && (G (! a1)) && (G (a2 -> (G (! a1))))) || ((G (! a1)) && (F (a1 && (F a0))) && (G (a1 && ((! a2) -> ((! a2) U (a0 && (! a2)))))) && (G (a0 -> (G a1)))))))";
    let f_no_par = "F a0 -> ((F a0 && (a1 U a0) && G (a2 && (! a1 -> (! a1 U (a0 && ! a1))))) -> (((a2 U a0) && G ! a1 && G (a2 -> G ! a1)) || (G ! a1 && F (a1 && F a0) && G (a1 && (! a2 -> (! a2 U (a0 && ! a2)))) && G (a0 -> G a1))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_9() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a3) -> (((F a3) && (a2 U a3) && (F a0) && (G (a1 && ((! a2) -> ((! a2) U ((! a2) && a3)))))) -> (((a2 U a3) && (G (a1 -> (G (! a0)))) && (G (! a3))) || ((G (! a0)) && (F (a3 && (F a1))) && (G (a1 && ((! a3) -> ((! a3) U (a2 && (! a3)))))) && (G (a1 -> (G a0))) && (G (! a1))))))";
    let f_no_par = "F a3 -> ((F a3 && (a2 U a3) && F a0 && G (a1 && (! a2 -> (! a2 U (! a2 && a3))))) -> (((a2 U a3) && G (a1 -> G ! a0) && G ! a3) || (G ! a0 && F (a3 && F a1) && G (a1 && (! a3 -> (! a3 U (a2 && ! a3)))) && G (a1 -> G a0) && G ! a1)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_10() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a0) -> (((F a0) && (a2 U a0) && (G (a4 && ((! a2) -> ((! a2) U ((! a2) && a3)))))) -> (((a2 U a0) && (G (a4 -> (G (! a3)))) && (G (! a2))) || ((G (! a3)) && (G (! a2)) && (F (a2 && (F a1))) && (G (a3 && ((! a4) -> ((! a4) U (a0 && (! a4)))))) && (G (a0 -> (G a3)))))))";
    let f_no_par = "F a0 -> ((F a0 && (a2 U a0) && G (a4 && (! a2 -> (! a2 U (! a2 && a3))))) -> (((a2 U a0) && G (a4 -> G ! a3) && G ! a2) || (G ! a3 && G ! a2 && F (a2 && F a1) && G (a3 && (! a4 -> (! a4 U (a0 && ! a4)))) && G (a0 -> G a3))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_11() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a0) -> (((F a0) && (a2 U a0) && (G (a4 && ((! a2) -> ((! a2) U ((! a2) && a5)))))) -> (((a5 U a0) && (G (a5 -> (G (! a3)))) && (G (! a2))) || ((G (! a3)) && (F (a2 && (F a1))) && (G (a3 && ((! a5) -> ((! a5) U (a0 && (! a5)))))) && (G (a4 -> (G a3))) && (G (! a4))))))";
    let f_no_par = "F a0 -> ((F a0 && (a2 U a0) && G (a4 && (! a2 -> (! a2 U (! a2 && a5))))) -> (((a5 U a0) && G (a5 -> G ! a3) && G ! a2) || (G ! a3 && F (a2 && F a1) && G (a3 && (! a5 -> (! a5 U (a0 && ! a5)))) && G (a4 -> G a3) && G ! a4)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_12() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a5) -> (((a2 U a5) && (F a6) && (G (a6 && ((! a0) -> ((! a0) U ((! a0) && a1))))) && (F a2)) -> (((a4 U a2) && (G (a2 -> (G (! a4)))) && (G (! a5))) || ((F (a5 && (F a0))) && (G (a3 && ((! a5) -> ((! a5) U ((! a5) && a6))))) && (G (a1 -> (G a2))) && (G (! a3)) && (G (! a6))))))";
    let f_no_par = "F a5 -> (((a2 U a5) && F a6 && G (a6 && (! a0 -> (! a0 U (! a0 && a1)))) && F a2) -> (((a4 U a2) && G (a2 -> G ! a4) && G ! a5) || (F (a5 && F a0) && G (a3 && (! a5 -> (! a5 U (! a5 && a6)))) && G (a1 -> G a2) && G ! a3 && G ! a6)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_13() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a7) -> (((F a7) && (a5 U a7) && (F a0) && (G (a0 && ((! a1) -> ((! a1) U ((! a1) && a4)))))) -> (((a4 U a7) && (G (a7 -> (G (! a5)))) && (G (! a1))) || ((G (! a1)) && (F (a1 && (F a3))) && (G (a7 && ((! a2) -> ((! a2) U (a1 && (! a2)))))) && (G (a3 -> (G a7)))))))";
    let f_no_par = "F a7 -> ((F a7 && (a5 U a7) && F a0 && G (a0 && (! a1 -> (! a1 U (! a1 && a4))))) -> (((a4 U a7) && G (a7 -> G ! a5) && G ! a1) || (G ! a1 && F (a1 && F a3) && G (a7 && (! a2 -> (! a2 U (a1 && ! a2)))) && G (a3 -> G a7))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_14() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a0) -> (((F a0) && (a5 U a0) && (G (a8 && ((! a4) -> ((! a4) U ((! a4) && a7))))) && (F a1)) -> (((a5 U a1) && (G (a7 -> (G (! a6)))) && (G (! a5))) || ((G (! a6)) && (F (a5 && (F a2))) && (G (a7 && ((! a8) -> ((! a8) U (a6 && (! a8)))))) && (G (a7 -> (G a1))) && (G (! a8))))))";
    let f_no_par = "F a0 -> ((F a0 && (a5 U a0) && G (a8 && (! a4 -> (! a4 U (! a4 && a7)))) && F a1) -> (((a5 U a1) && G (a7 -> G ! a6) && G ! a5) || (G ! a6 && F (a5 && F a2) && G (a7 && (! a8 -> (! a8 U (a6 && ! a8)))) && G (a7 -> G a1) && G ! a8)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_15() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a9) -> (((a0 U a9) && (F a2) && (G (a4 && ((! a8) -> ((! a8) U (a5 && (! a8)))))) && (F a6)) -> (((a1 U a6) && (G (a5 -> (G (! a7)))) && (G (! a2))) || ((F (a2 && (F a7))) && (G (a7 && ((! a1) -> ((! a1) U ((! a1) && a9))))) && (G (a1 -> (G a8))) && (G (! a6)) && (G (! a5)) && (G (a4 -> (G (! a7))))))))";
    let f_no_par = "F a9 -> (((a0 U a9) && F a2 && G (a4 && (! a8 -> (! a8 U (a5 && ! a8)))) && F a6) -> (((a1 U a6) && G (a5 -> G ! a7) && G ! a2) || (F (a2 && F a7) && G (a7 && (! a1 -> (! a1 U (! a1 && a9)))) && G (a1 -> G a8) && G ! a6 && G ! a5 && G (a4 -> G ! a7))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_16() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a10) -> (((a0 U a10) && (F a2) && (G (a4 && ((! a8) -> ((! a8) U (a5 && (! a8)))))) && (F a6)) -> (((a1 U a6) && (G (a5 -> (G (! a7)))) && (G (! a2))) || ((F (a2 && (F a7))) && (G (a7 && ((! a1) -> ((! a1) U ((! a1) && a10))))) && (G (a9 -> (G a8))) && (G (! a1)) && (G (! a3)) && (G (a7 -> (G (! a8))))))))";
    let f_no_par = "F a10 -> (((a0 U a10) && F a2 && G (a4 && (! a8 -> (! a8 U (a5 && ! a8)))) && F a6) -> (((a1 U a6) && G (a5 -> G ! a7) && G ! a2) || (F (a2 && F a7) && G (a7 && (! a1 -> (! a1 U (! a1 && a10)))) && G (a9 -> G a8) && G ! a1 && G ! a3 && G (a7 -> G ! a8))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_17() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10", "a11",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a11) -> (((a0 U a11) && (F a2) && (G (a4 && ((! a8) -> ((! a8) U (a5 && (! a8)))))) && (F a1)) -> (((a11 U a1) && (G (a7 -> (G (! a6)))) && (G (! a5))) || ((G (! a6)) && (F (a5 && (F a2))) && (G (a7 && ((! a11) -> ((! a11) U (a1 && (! a11)))))) && (G (a8 -> (G a7))) && (G (! a9)) && (G (a8 -> (G (! a5))))))))";
    let f_no_par = "F a11 -> (((a0 U a11) && F a2 && G (a4 && (! a8 -> (! a8 U (a5 && ! a8)))) && F a1) -> (((a11 U a1) && G (a7 -> G ! a6) && G ! a5) || (G ! a6 && F (a5 && F a2) && G (a7 && (! a11 -> (! a11 U (a1 && ! a11)))) && G (a8 -> G a7) && G ! a9 && G (a8 -> G ! a5))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_18() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10", "a11", "a12",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a12) -> (((a0 U a12) && (F a2) && (G (a4 && ((! a8) -> ((! a8) U (a5 && (! a8)))))) && (F a1)) -> (((a11 U a1) && (G (a11 -> (G (! a6)))) && (G (! a7))) || ((F (a7 && (F a5))) && (G (a2 && ((! a7) -> ((! a7) U ((! a7) && a11))))) && (G (a7 -> (G a1))) && (G (! a11)) && (G (! a8)) && (G (a6 -> (G (! a1))))))))";
    let f_no_par = "F a12 -> (((a0 U a12) && F a2 && G (a4 && (! a8 -> (! a8 U (a5 && ! a8)))) && F a1) -> (((a11 U a1) && G (a11 -> G ! a6) && G ! a7) || (F (a7 && F a5) && G (a2 && (! a7 -> (! a7 U (! a7 && a11)))) && G (a7 -> G a1) && G ! a11 && G ! a8 && G (a6 -> G ! a1))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_19() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10", "a11", "a12", "a13",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a0) -> (((a13 U a0) && (F a13) && (G (a2 && ((! a8) -> ((! a8) U (a4 && (! a8)))))) && (F a5)) -> (((a8 U a5) && (G (a12 -> (G (! a11)))) && (G (! a1))) || ((F (a1 && (F a6))) && (G (a11 && ((! a7) -> ((! a7) U (a5 && (! a7)))))) && (G (a7 -> (G a2))) && (G (! a7)) && (G (a8 -> (G (! a11))))))))";
    let f_no_par = "F a0 -> (((a13 U a0) && F a13 && G (a2 && (! a8 -> (! a8 U (a4 && ! a8)))) && F a5) -> (((a8 U a5) && G (a12 -> G ! a11) && G ! a1) || (F (a1 && F a6) && G (a11 && (! a7 -> (! a7 U (a5 && ! a7)))) && G (a7 -> G a2) && G ! a7 && G (a8 -> G ! a11))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_20() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a2) -> (((F a2) && (a0 U a2) && (F a0) && (G (a2 && ((! a1) -> ((! a1) U (a0 && (! a1))))))) -> (((a1 U a2) && (G (! a1)) && (G (a0 -> (G (! a1))))) || ((G (! a1)) && (F (a1 && (F a2))) && (G (a0 && ((! a1) -> ((! a1) U ((! a1) && a2))))) && (G (a0 -> (G a1))) && (G (! a2)) && (G (a2 -> (G (! a0))))))))";
    let f_no_par = "F a2 -> ((F a2 && (a0 U a2) && F a0 && G (a2 && (! a1 -> (! a1 U (a0 && ! a1))))) -> (((a1 U a2) && G ! a1 && G (a0 -> G ! a1)) || (G ! a1 && F (a1 && F a2) && G (a0 && (! a1 -> (! a1 U (! a1 && a2)))) && G (a0 -> G a1) && G ! a2 && G (a2 -> G ! a0))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_21() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a3) -> (((a0 U a3) && (F a1) && (G (a2 && ((! a3) -> ((! a3) U (a1 && (! a3))))))) -> (((a0 U a1) && (G (a1 -> (G (! a3)))) && (G (! a1))) || ((G (! a3)) && (G (! a1)) && (F (a1 && (F a3))) && (G (a3 && ((! a0) -> ((! a0) U ((! a0) && a1))))) && (G (a1 -> (G a0))) && (G (a0 -> (G (! a2))))))))";
    let f_no_par = "F a3 -> (((a0 U a3) && F a1 && G (a2 && (! a3 -> (! a3 U (a1 && ! a3))))) -> (((a0 U a1) && G (a1 -> G ! a3) && G ! a1) || (G ! a3 && G ! a1 && F (a1 && F a3) && G (a3 && (! a0 -> (! a0 U (! a0 && a1)))) && G (a1 -> G a0) && G (a0 -> G ! a2))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_22() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a4) -> (((a0 U a4) && (F a1) && (G (a4 && ((! a2) -> ((! a2) U ((! a2) && a3))))) && (F a3)) -> (((a0 U a3) && (G (a2 -> (G (! a3)))) && (G (! a1))) || ((G (! a3)) && (F (a1 && (F a3))) && (G (a3 && ((! a0) -> ((! a0) U ((! a0) && a1))))) && (G (a0 -> (G a4))) && (G (! a2)) && (G (a1 -> (G (! a2))))))))";
    let f_no_par = "F a4 -> (((a0 U a4) && F a1 && G (a4 && (! a2 -> (! a2 U (! a2 && a3)))) && F a3) -> (((a0 U a3) && G (a2 -> G ! a3) && G ! a1) || (G ! a3 && F (a1 && F a3) && G (a3 && (! a0 -> (! a0 U (! a0 && a1)))) && G (a0 -> G a4) && G ! a2 && G (a1 -> G ! a2))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_23() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a5) -> (((a0 U a5) && (F a1) && (G (a2 && ((! a4) -> ((! a4) U ((! a4) && a5))))) && (F a0)) -> (((a5 U a0) && (G (a5 -> (G (! a3)))) && (G (! a2))) || ((G (! a3)) && (F (a2 && (F a1))) && (G (a3 && ((! a5) -> ((! a5) U (a0 && (! a5)))))) && (G (a4 -> (G a3))) && (G (! a4)) && (G (a4 -> (G (! a2))))))))";
    let f_no_par = "F a5 -> (((a0 U a5) && F a1 && G (a2 && (! a4 -> (! a4 U (! a4 && a5)))) && F a0) -> (((a5 U a0) && G (a5 -> G ! a3) && G ! a2) || (G ! a3 && F (a2 && F a1) && G (a3 && (! a5 -> (! a5 U (a0 && ! a5)))) && G (a4 -> G a3) && G ! a4 && G (a4 -> G ! a2))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_24() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a0) -> (((a6 U a0) && (F a6) && (G (a1 && ((! a4) -> ((! a4) U (a2 && (! a4)))))) && (F a2)) -> (((a4 U a2) && (G (a0 -> (G (! a5)))) && (G (! a3))) || ((G (! a3)) && (F (a3 && (F a5))) && (G (a3 && ((! a2) -> ((! a2) U (a1 && (! a2)))))) && (G (a6 -> (G a3))) && (G (! a6)) && (G (a4 -> (G (! a5))))))))";
    let f_no_par = "F a0 -> (((a6 U a0) && F a6 && G (a1 && (! a4 -> (! a4 U (a2 && ! a4)))) && F a2) -> (((a4 U a2) && G (a0 -> G ! a5) && G ! a3) || (G ! a3 && F (a3 && F a5) && G (a3 && (! a2 -> (! a2 U (a1 && ! a2)))) && G (a6 -> G a3) && G ! a6 && G (a4 -> G ! a5))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_25() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a6) -> (((a0 U a6) && (F a0) && (G (a4 && ((! a7) -> ((! a7) U (a2 && (! a7)))))) && (F a3)) -> (((a1 U a3) && (G (a2 -> (G (! a7)))) && (G (! a2))) || ((F (a2 && (F a3))) && (G (a7 && ((! a6) -> ((! a6) U (a0 && (! a6)))))) && (G (a5 -> (G a7))) && (G (! a1)) && (G (! a3)) && (G (a6 -> (G (! a7))))))))";
    let f_no_par = "F a6 -> (((a0 U a6) && F a0 && G (a4 && (! a7 -> (! a7 U (a2 && ! a7)))) && F a3) -> (((a1 U a3) && G (a2 -> G ! a7) && G ! a2) || (F (a2 && F a3) && G (a7 && (! a6 -> (! a6 U (a0 && ! a6)))) && G (a5 -> G a7) && G ! a1 && G ! a3 && G (a6 -> G ! a7))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_26() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a8) -> (((a0 U a8) && (F a2) && (G (a8 && ((! a5) -> ((! a5) U ((! a5) && a7))))) && (F a6)) -> (((a1 U a6) && (G (a5 -> (G (! a7)))) && (G (! a2))) || ((F (a2 && (F a7))) && (G (a7 && ((! a1) -> ((! a1) U ((! a1) && a3))))) && (G (a1 -> (G a8))) && (G (! a6)) && (G (! a5)) && (G (a2 -> (G (! a4))))))))";
    let f_no_par = "F a8 -> (((a0 U a8) && F a2 && G (a8 && (! a5 -> (! a5 U (! a5 && a7)))) && F a6) -> (((a1 U a6) && G (a5 -> G ! a7) && G ! a2) || (F (a2 && F a7) && G (a7 && (! a1 -> (! a1 U (! a1 && a3)))) && G (a1 -> G a8) && G ! a6 && G ! a5 && G (a2 -> G ! a4))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_27() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a2) -> (((a0 U a2) && (F a8) && (G (a8 && ((! a5) -> ((! a5) U (a1 && (! a5)))))) && (F a7)) -> ((((a6 U a7) && (G (a2 -> (G (! a5)))) && (G (! a7))) || ((G (! a5)) && (G (! a7)) && (F (a7 && (F a9))) && (G (a1 && ((! a7) -> ((! a7) U ((! a7) && a9))))) && (G (a3 -> (G a6))) && (G (a1 -> (G (! a2)))) && (F a5))) -> (a9 U a5))))";
    let f_no_par = "F a2 -> (((a0 U a2) && F a8 && G (a8 && (! a5 -> (! a5 U (a1 && ! a5)))) && F a7) -> ((((a6 U a7) && G (a2 -> G ! a5) && G ! a7) || (G ! a5 && G ! a7 && F (a7 && F a9) && G (a1 && (! a7 -> (! a7 U (! a7 && a9)))) && G (a3 -> G a6) && G (a1 -> G ! a2) && F a5)) -> (a9 U a5)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_28() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a2) -> (((a0 U a2) && (F a8) && (G (a8 && ((! a5) -> ((! a5) U (a1 && (! a5)))))) && (F a7)) -> ((((a6 U a7) && (G (a2 -> (G (! a5)))) && (G (! a7))) || ((G (! a5)) && (F (a7 && (F a10))) && (G (a1 && ((! a7) -> ((! a7) U ((! a7) && a8))))) && (G (a1 -> (G a9))) && (G (! a6)) && (G (a4 -> (G (! a7)))) && (F a1))) -> (a2 U a1))))";
    let f_no_par = "F a2 -> (((a0 U a2) && F a8 && G (a8 && (! a5 -> (! a5 U (a1 && ! a5)))) && F a7) -> ((((a6 U a7) && G (a2 -> G ! a5) && G ! a7) || (G ! a5 && F (a7 && F a10) && G (a1 && (! a7 -> (! a7 U (! a7 && a8)))) && G (a1 -> G a9) && G ! a6 && G (a4 -> G ! a7) && F a1)) -> (a2 U a1)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_29() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10", "a11",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a2) -> (((a0 U a2) && (F a8) && (G (a8 && ((! a5) -> ((! a5) U (a1 && (! a5)))))) && (F a7)) -> ((((a6 U a7) && (G (a2 -> (G (! a5)))) && (G (! a7))) || ((G (! a5)) && (F (a7 && (F a11))) && (G (a1 && ((! a7) -> ((! a7) U ((! a7) && a8))))) && (G (a1 -> (G a9))) && (G (! a6)) && (G (a4 -> (G (! a7)))) && (F a1))) -> (a2 U a1))))";
    let f_no_par = "F a2 -> (((a0 U a2) && F a8 && G (a8 && (! a5 -> (! a5 U (a1 && ! a5)))) && F a7) -> ((((a6 U a7) && G (a2 -> G ! a5) && G ! a7) || (G ! a5 && F (a7 && F a11) && G (a1 && (! a7 -> (! a7 U (! a7 && a8)))) && G (a1 -> G a9) && G ! a6 && G (a4 -> G ! a7) && F a1)) -> (a2 U a1)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_30() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10", "a11", "a12",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a2) -> (((a0 U a2) && (F a8) && (G (a8 && ((! a5) -> ((! a5) U (a1 && (! a5)))))) && (F a11)) -> ((((a6 U a11) && (G (a5 -> (G (! a7)))) && (G (! a2))) || ((F a7) && (F (a2 && (F a7))) && (G (a7 && ((! a1) -> ((! a1) U ((! a1) && a12))))) && (G (a12 -> (G a11))) && (G (! a8)) && (G (! a1)) && (G (a5 -> (G (! a3)))))) -> (a8 U a7))))";
    let f_no_par = "F a2 -> (((a0 U a2) && F a8 && G (a8 && (! a5 -> (! a5 U (a1 && ! a5)))) && F a11) -> ((((a6 U a11) && G (a5 -> G ! a7) && G ! a2) || (F a7 && F (a2 && F a7) && G (a7 && (! a1 -> (! a1 U (! a1 && a12)))) && G (a12 -> G a11) && G ! a8 && G ! a1 && G (a5 -> G ! a3))) -> (a8 U a7)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_31() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10", "a11", "a12", "a13",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a0) -> (((a13 U a0) && (F a2) && (G (a4 && ((! a8) -> ((! a8) U (a5 && (! a8)))))) && (F a12)) -> ((((a11 U a12) && (G (a6 -> (G (! a1)))) && (G (! a11))) || ((G (! a11)) && (F (a11 && (F a7))) && (G (a5 && ((! a2) -> ((! a2) U ((! a2) && a7))))) && (G (a1 -> (G a7))) && (G (! a7)) && (G (a1 -> (G (! a9)))) && (F a3))) -> (a6 U a3))))";
    let f_no_par = "F a0 -> (((a13 U a0) && F a2 && G (a4 && (! a8 -> (! a8 U (a5 && ! a8)))) && F a12) -> ((((a11 U a12) && G (a6 -> G ! a1) && G ! a11) || (G ! a11 && F (a11 && F a7) && G (a5 && (! a2 -> (! a2 U (! a2 && a7)))) && G (a1 -> G a7) && G ! a7 && G (a1 -> G ! a9) && F a3)) -> (a6 U a3)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_32() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a2) -> (((F a2) && (a0 U a2) && (G (a2 && ((! a1) -> ((! a1) U (a0 && (! a1))))))) -> ((((a1 U a2) && (G (! a1)) && (G (a0 -> (G (! a1))))) || ((G (! a1)) && (F (a1 && (F a2))) && (G (a0 && ((! a1) -> ((! a1) U ((! a1) && a2))))) && (G (a0 -> (G a1))) && (G (! a2)) && (G (a2 -> (G (! a0)))) && (F a1))) -> (a2 U a1))))";
    let f_no_par = "F a2 -> ((F a2 && (a0 U a2) && G (a2 && (! a1 -> (! a1 U (a0 && ! a1))))) -> ((((a1 U a2) && G ! a1 && G (a0 -> G ! a1)) || (G ! a1 && F (a1 && F a2) && G (a0 && (! a1 -> (! a1 U (! a1 && a2)))) && G (a0 -> G a1) && G ! a2 && G (a2 -> G ! a0) && F a1)) -> (a2 U a1)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_33() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a3) -> (((a0 U a3) && (F a2) && (G (a2 && ((! a3) -> ((! a3) U (a0 && (! a3))))))) -> ((((a3 U a2) && (G (a1 -> (G (! a3)))) && (G (! a1))) || ((F a2) && (G (! a3)) && (G (! a1)) && (F (a1 && (F a3))) && (G (a3 && ((! a0) -> ((! a0) U ((! a0) && a1))))) && (G (a1 -> (G a0))) && (G (a0 -> (G (! a2)))))) -> (a0 U a2))))";
    let f_no_par = "F a3 -> (((a0 U a3) && F a2 && G (a2 && (! a3 -> (! a3 U (a0 && ! a3))))) -> ((((a3 U a2) && G (a1 -> G ! a3) && G ! a1) || (F a2 && G ! a3 && G ! a1 && F (a1 && F a3) && G (a3 && (! a0 -> (! a0 U (! a0 && a1)))) && G (a1 -> G a0) && G (a0 -> G ! a2))) -> (a0 U a2)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_34() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a1) -> (((a0 U a1) && (F a4) && (G (a4 && ((! a2) -> ((! a2) U ((! a2) && a3))))) && (F a3)) -> ((((a0 U a3) && (G (a2 -> (G (! a3)))) && (G (! a1))) || ((G (! a3)) && (F (a1 && (F a3))) && (G (a3 && ((! a0) -> ((! a0) U ((! a0) && a1))))) && (G (a0 -> (G a4))) && (G (! a2)) && (G (a1 -> (G (! a2)))) && (F a2))) -> (a0 U a2))))";
    let f_no_par = "F a1 -> (((a0 U a1) && F a4 && G (a4 && (! a2 -> (! a2 U (! a2 && a3)))) && F a3) -> ((((a0 U a3) && G (a2 -> G ! a3) && G ! a1) || (G ! a3 && F (a1 && F a3) && G (a3 && (! a0 -> (! a0 U (! a0 && a1)))) && G (a0 -> G a4) && G ! a2 && G (a1 -> G ! a2) && F a2)) -> (a0 U a2)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_35() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a1) -> (((a0 U a1) && (F a4) && (G (a4 && ((! a2) -> ((! a2) U (a0 && (! a2)))))) && (F a5)) -> ((((a3 U a5) && (G (a1 -> (G (! a2)))) && (G (! a3))) || ((G (! a2)) && (G (! a3)) && (F (a3 && (F a5))) && (G (a0 && ((! a3) -> ((! a3) U ((! a3) && a5))))) && (G (a1 -> (G a3))) && (G (a0 -> (G (! a1)))) && (F a2))) -> (a4 U a2))))";
    let f_no_par = "F a1 -> (((a0 U a1) && F a4 && G (a4 && (! a2 -> (! a2 U (a0 && ! a2)))) && F a5) -> ((((a3 U a5) && G (a1 -> G ! a2) && G ! a3) || (G ! a2 && G ! a3 && F (a3 && F a5) && G (a0 && (! a3 -> (! a3 U (! a3 && a5)))) && G (a1 -> G a3) && G (a0 -> G ! a1) && F a2)) -> (a4 U a2)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_36() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a0) -> (((F a0) && (a6 U a0) && (F a1) && (G (a2 && ((! a4) -> ((! a4) U ((! a4) && a6)))))) -> ((((a5 U a0) && (G (! a3)) && (G (a5 -> (G (! a3))))) || ((F a1) && (G (! a3)) && (F (a3 && (F a2))) && (G (a1 && ((! a3) -> ((! a3) U ((! a3) && a5))))) && (G (a0 -> (G a6))) && (G (! a5)) && (G (a0 -> (G (! a4)))))) -> (a3 U a1))))";
    let f_no_par = "F a0 -> ((F a0 && (a6 U a0) && F a1 && G (a2 && (! a4 -> (! a4 U (! a4 && a6))))) -> ((((a5 U a0) && G ! a3 && G (a5 -> G ! a3)) || (F a1 && G ! a3 && F (a3 && F a2) && G (a1 && (! a3 -> (! a3 U (! a3 && a5)))) && G (a0 -> G a6) && G ! a5 && G (a0 -> G ! a4))) -> (a3 U a1)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_37() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a1) -> (((a0 U a1) && (F a4) && (G (a5 && ((! a7) -> ((! a7) U (a0 && (! a7)))))) && (F a5)) -> ((((a6 U a5) && (G (a2 -> (G (! a7)))) && (G (! a2))) || ((F a1) && (F (a2 && (F a3))) && (G (a7 && ((! a6) -> ((! a6) U (a0 && (! a6)))))) && (G (a5 -> (G a7))) && (G (! a1)) && (G (! a3)) && (G (a6 -> (G (! a7)))))) -> (a4 U a1))))";
    let f_no_par = "F a1 -> (((a0 U a1) && F a4 && G (a5 && (! a7 -> (! a7 U (a0 && ! a7)))) && F a5) -> ((((a6 U a5) && G (a2 -> G ! a7) && G ! a2) || (F a1 && F (a2 && F a3) && G (a7 && (! a6 -> (! a6 U (a0 && ! a6)))) && G (a5 -> G a7) && G ! a1 && G ! a3 && G (a6 -> G ! a7))) -> (a4 U a1)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_38() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a2) -> (((a0 U a2) && (F a8) && (G (a8 && ((! a5) -> ((! a5) U ((! a5) && a7))))) && (F a6)) -> ((((a1 U a6) && (G (a5 -> (G (! a7)))) && (G (! a2))) || ((F (a2 && (F a7))) && (G (a7 && ((! a1) -> ((! a1) U ((! a1) && a3))))) && (G (a1 -> (G a8))) && (G (! a6)) && (G (! a5)) && (G (a2 -> (G (! a4)))) && (F a5))) -> (a1 U a5))))";
    let f_no_par = "F a2 -> (((a0 U a2) && F a8 && G (a8 && (! a5 -> (! a5 U (! a5 && a7)))) && F a6) -> ((((a1 U a6) && G (a5 -> G ! a7) && G ! a2) || (F (a2 && F a7) && G (a7 && (! a1 -> (! a1 U (! a1 && a3)))) && G (a1 -> G a8) && G ! a6 && G ! a5 && G (a2 -> G ! a4) && F a5)) -> (a1 U a5)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_39() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a8) -> (((a2 U a8) && (F a4) && (G (a5 && ((! a1) -> ((! a1) U ((! a1) && a6))))) && (F a5)) -> ((((a7 U a5) && (G (a7 -> (G (! a2)))) && (G (! a7))) || ((F a5) && (G (! a7)) && (F (a7 && (F a1))) && (G (a7 && ((! a8) -> ((! a8) U (a1 && (! a8)))))) && (G (a3 -> (G a6))) && (G (! a5)) && (G (a1 -> (G (! a2)))))) -> (((F a5) && (a9 U a5)) -> (a1 U a5)))))";
    let f_no_par = "F a8 -> (((a2 U a8) && F a4 && G (a5 && (! a1 -> (! a1 U (! a1 && a6)))) && F a5) -> ((((a7 U a5) && G (a7 -> G ! a2) && G ! a7) || (F a5 && G ! a7 && F (a7 && F a1) && G (a7 && (! a8 -> (! a8 U (a1 && ! a8)))) && G (a3 -> G a6) && G ! a5 && G (a1 -> G ! a2))) -> ((F a5 && (a9 U a5)) -> (a1 U a5))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_40() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a8) -> (((a2 U a8) && (F a4) && (G (a5 && ((! a1) -> ((! a1) U ((! a1) && a6))))) && (F a5)) -> ((((a7 U a5) && (G (a7 -> (G (! a2)))) && (G (! a7))) || ((F a5) && (G (! a7)) && (F (a7 && (F a1))) && (G (a7 && ((! a8) -> ((! a8) U (a1 && (! a8)))))) && (G (a3 -> (G a6))) && (G (! a5)) && (G (a1 -> (G (! a2)))))) -> (((a9 U a5) && (F a9)) -> (a1 U a9)))))";
    let f_no_par = "F a8 -> (((a2 U a8) && F a4 && G (a5 && (! a1 -> (! a1 U (! a1 && a6)))) && F a5) -> ((((a7 U a5) && G (a7 -> G ! a2) && G ! a7) || (F a5 && G ! a7 && F (a7 && F a1) && G (a7 && (! a8 -> (! a8 U (a1 && ! a8)))) && G (a3 -> G a6) && G ! a5 && G (a1 -> G ! a2))) -> (((a9 U a5) && F a9) -> (a1 U a9))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_41() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10", "a11",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a8) -> (((a2 U a8) && (F a4) && (G (a5 && ((! a1) -> ((! a1) U ((! a1) && a6))))) && (F a7)) -> ((((a11 U a7) && (G (a2 -> (G (! a5)))) && (G (! a7))) || ((G (! a5)) && (F (a7 && (F a11))) && (G (a1 && ((! a7) -> ((! a7) U ((! a7) && a8))))) && (G (a1 -> (G a9))) && (G (! a6)) && (G (a4 -> (G (! a7)))) && (F a1))) -> (((a2 U a1) && (F a5)) -> (a9 U a5)))))";
    let f_no_par = "F a8 -> (((a2 U a8) && F a4 && G (a5 && (! a1 -> (! a1 U (! a1 && a6)))) && F a7) -> ((((a11 U a7) && G (a2 -> G ! a5) && G ! a7) || (G ! a5 && F (a7 && F a11) && G (a1 && (! a7 -> (! a7 U (! a7 && a8)))) && G (a1 -> G a9) && G ! a6 && G (a4 -> G ! a7) && F a1)) -> (((a2 U a1) && F a5) -> (a9 U a5))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_42() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10", "a11", "a12",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a8) -> (((a2 U a8) && (F a4) && (G (a5 && ((! a11) -> ((! a11) U (a1 && (! a11)))))) && (F a11)) -> ((((a6 U a11) && (G (a5 -> (G (! a7)))) && (G (! a2))) || ((F a7) && (F (a2 && (F a7))) && (G (a7 && ((! a1) -> ((! a1) U ((! a1) && a12))))) && (G (a12 -> (G a11))) && (G (! a8)) && (G (! a1)) && (G (a5 -> (G (! a3)))))) -> (((F a11) && (a8 U a7)) -> (a12 U a11)))))";
    let f_no_par = "F a8 -> (((a2 U a8) && F a4 && G (a5 && (! a11 -> (! a11 U (a1 && ! a11)))) && F a11) -> ((((a6 U a11) && G (a5 -> G ! a7) && G ! a2) || (F a7 && F (a2 && F a7) && G (a7 && (! a1 -> (! a1 U (! a1 && a12)))) && G (a12 -> G a11) && G ! a8 && G ! a1 && G (a5 -> G ! a3))) -> ((F a11 && (a8 U a7)) -> (a12 U a11))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_43() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10", "a11", "a12", "a13",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a8) -> (((a2 U a8) && (F a4) && (G (a5 && ((! a11) -> ((! a11) U (a1 && (! a11)))))) && (F a11)) -> ((((a6 U a11) && (G (a5 -> (G (! a7)))) && (G (! a2))) || ((F a7) && (F (a2 && (F a7))) && (G (a7 && ((! a1) -> ((! a1) U ((! a1) && a13))))) && (G (a13 -> (G a11))) && (G (! a8)) && (G (! a1)) && (G (a5 -> (G (! a3)))))) -> (((F a11) && (a8 U a7)) -> (a12 U a11)))))";
    let f_no_par = "F a8 -> (((a2 U a8) && F a4 && G (a5 && (! a11 -> (! a11 U (a1 && ! a11)))) && F a11) -> ((((a6 U a11) && G (a5 -> G ! a7) && G ! a2) || (F a7 && F (a2 && F a7) && G (a7 && (! a1 -> (! a1 U (! a1 && a13)))) && G (a13 -> G a11) && G ! a8 && G ! a1 && G (a5 -> G ! a3))) -> ((F a11 && (a8 U a7)) -> (a12 U a11))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_44() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a1) -> (((F a1) && (a0 U a1) && (F a2) && (G (a2 && ((! a0) -> ((! a0) U ((! a0) && a1)))))) -> ((((a2 U a1) && (G (! a1)) && (G (a0 -> (G (! a1))))) || ((F a1) && (G (! a1)) && (F (a1 && (F a2))) && (G (a0 && ((! a1) -> ((! a1) U ((! a1) && a2))))) && (G (a0 -> (G a1))) && (G (! a2)) && (G (a2 -> (G (! a0)))))) -> (((F a1) && (a2 U a1)) -> (a0 U a1)))))";
    let f_no_par = "F a1 -> ((F a1 && (a0 U a1) && F a2 && G (a2 && (! a0 -> (! a0 U (! a0 && a1))))) -> ((((a2 U a1) && G ! a1 && G (a0 -> G ! a1)) || (F a1 && G ! a1 && F (a1 && F a2) && G (a0 && (! a1 -> (! a1 U (! a1 && a2)))) && G (a0 -> G a1) && G ! a2 && G (a2 -> G ! a0))) -> ((F a1 && (a2 U a1)) -> (a0 U a1))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_45() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a2) -> (((F a2) && (a1 U a2) && (G (a2 && ((! a3) -> ((! a3) U (a0 && (! a3))))))) -> ((((a3 U a2) && (G (a1 -> (G (! a3)))) && (G (! a1))) || ((F a2) && (G (! a3)) && (G (! a1)) && (F (a1 && (F a3))) && (G (a3 && ((! a0) -> ((! a0) U ((! a0) && a1))))) && (G (a1 -> (G a0))) && (G (a0 -> (G (! a2)))))) -> (((a0 U a2) && (F a0)) -> (a2 U a0)))))";
    let f_no_par = "F a2 -> ((F a2 && (a1 U a2) && G (a2 && (! a3 -> (! a3 U (a0 && ! a3))))) -> ((((a3 U a2) && G (a1 -> G ! a3) && G ! a1) || (F a2 && G ! a3 && G ! a1 && F (a1 && F a3) && G (a3 && (! a0 -> (! a0 U (! a0 && a1)))) && G (a1 -> G a0) && G (a0 -> G ! a2))) -> (((a0 U a2) && F a0) -> (a2 U a0))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_46() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a2) -> (((F a2) && (a1 U a2) && (F a4) && (G (a0 && ((! a3) -> ((! a3) U (a2 && (! a3))))))) -> ((((a3 U a2) && (G (a3 -> (G (! a1)))) && (G (! a3))) || ((F a2) && (F (a3 && (F a0))) && (G (a3 && ((! a0) -> ((! a0) U ((! a0) && a1))))) && (G (a2 -> (G a1))) && (G (! a4)) && (G (! a2)) && (G (a2 -> (G (! a0)))))) -> (((F a4) && (a0 U a2)) -> (a3 U a4)))))";
    let f_no_par = "F a2 -> ((F a2 && (a1 U a2) && F a4 && G (a0 && (! a3 -> (! a3 U (a2 && ! a3))))) -> ((((a3 U a2) && G (a3 -> G ! a1) && G ! a3) || (F a2 && F (a3 && F a0) && G (a3 && (! a0 -> (! a0 U (! a0 && a1)))) && G (a2 -> G a1) && G ! a4 && G ! a2 && G (a2 -> G ! a0))) -> ((F a4 && (a0 U a2)) -> (a3 U a4))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_47() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a4) -> (((a1 U a4) && (F a2) && (G (a2 && ((! a0) -> ((! a0) U ((! a0) && a3))))) && (F a3)) -> ((((a5 U a3) && (G (a1 -> (G (! a2)))) && (G (! a3))) || ((F a2) && (G (! a2)) && (G (! a3)) && (F (a3 && (F a5))) && (G (a0 && ((! a3) -> ((! a3) U ((! a3) && a5))))) && (G (a1 -> (G a3))) && (G (a0 -> (G (! a1)))))) -> (((F a4) && (a4 U a2)) -> (a0 U a4)))))";
    let f_no_par = "F a4 -> (((a1 U a4) && F a2 && G (a2 && (! a0 -> (! a0 U (! a0 && a3)))) && F a3) -> ((((a5 U a3) && G (a1 -> G ! a2) && G ! a3) || (F a2 && G ! a2 && G ! a3 && F (a3 && F a5) && G (a0 && (! a3 -> (! a3 U (! a3 && a5)))) && G (a1 -> G a3) && G (a0 -> G ! a1))) -> ((F a4 && (a4 U a2)) -> (a0 U a4))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_48() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a4) -> (((a1 U a4) && (F a2) && (G (a2 && ((! a5) -> ((! a5) U (a0 && (! a5)))))) && (F a5)) -> ((((a3 U a5) && (G (a2 -> (G (! a3)))) && (G (! a1))) || ((F a3) && (F (a1 && (F a3))) && (G (a3 && ((! a0) -> ((! a0) U ((! a0) && a6))))) && (G (a6 -> (G a5))) && (G (! a4)) && (G (! a0)) && (G (a2 -> (G (! a1)))))) -> (((F a5) && (a4 U a3)) -> (a6 U a5)))))";
    let f_no_par = "F a4 -> (((a1 U a4) && F a2 && G (a2 && (! a5 -> (! a5 U (a0 && ! a5)))) && F a5) -> ((((a3 U a5) && G (a2 -> G ! a3) && G ! a1) || (F a3 && F (a1 && F a3) && G (a3 && (! a0 -> (! a0 U (! a0 && a6)))) && G (a6 -> G a5) && G ! a4 && G ! a0 && G (a2 -> G ! a1))) -> ((F a5 && (a4 U a3)) -> (a6 U a5))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_49() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a4) -> (((F a4) && (a2 U a4) && (G (a5 && ((! a7) -> ((! a7) U (a0 && (! a7)))))) && (F a5)) -> ((((a6 U a5) && (G (a2 -> (G (! a7)))) && (G (! a2))) || ((F (a2 && (F a3))) && (G (a7 && ((! a6) -> ((! a6) U (a0 && (! a6)))))) && (G (a5 -> (G a7))) && (G (! a1)) && (G (! a3)) && (G (a6 -> (G (! a7)))) && (F a1))) -> (((F a4) && (a4 U a1)) -> (a1 U a4)))))";
    let f_no_par = "F a4 -> ((F a4 && (a2 U a4) && G (a5 && (! a7 -> (! a7 U (a0 && ! a7)))) && F a5) -> ((((a6 U a5) && G (a2 -> G ! a7) && G ! a2) || (F (a2 && F a3) && G (a7 && (! a6 -> (! a6 U (a0 && ! a6)))) && G (a5 -> G a7) && G ! a1 && G ! a3 && G (a6 -> G ! a7) && F a1)) -> ((F a4 && (a4 U a1)) -> (a1 U a4))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_50() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a4) -> (((a2 U a4) && (F a8) && (G (a1 && ((! a6) -> ((! a6) U (a5 && (! a6)))))) && (F a5)) -> ((((a7 U a5) && (G (a7 -> (G (! a2)))) && (G (! a7))) || ((F a5) && (F (a7 && (F a1))) && (G (a7 && ((! a1) -> ((! a1) U ((! a1) && a3))))) && (G (a5 -> (G a3))) && (G (! a8)) && (G (! a4)) && (G (a5 -> (G (! a1)))))) -> (((a1 U a5) && (F a6)) -> (a7 U a6)))))";
    let f_no_par = "F a4 -> (((a2 U a4) && F a8 && G (a1 && (! a6 -> (! a6 U (a5 && ! a6)))) && F a5) -> ((((a7 U a5) && G (a7 -> G ! a2) && G ! a7) || (F a5 && F (a7 && F a1) && G (a7 && (! a1 -> (! a1 U (! a1 && a3)))) && G (a5 -> G a3) && G ! a8 && G ! a4 && G (a5 -> G ! a1))) -> (((a1 U a5) && F a6) -> (a7 U a6))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_51() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a8) -> (((a4 U a8) && (F a5) && (G (a6 && ((! a7) -> ((! a7) U (a5 && (! a7)))))) && (F a7)) -> ((((a2 U a7) && (G (! a7)) && (G (a1 -> (G (! a7))))) || ((F a5) && (F (a7 && (F a8))) && (G (a9 && ((! a1) -> ((! a1) U ((! a1) && a6))))) && (G (a5 -> (G a3))) && (G (! a8)) && (G (! a4)) && (G (a5 -> (G (! a1)))))) -> (((a1 U a5) && (F a6)) -> (((a7 U a6) && (F a2)) -> ((! a2) U a6))))))";
    let f_no_par = "F a8 -> (((a4 U a8) && F a5 && G (a6 && (! a7 -> (! a7 U (a5 && ! a7)))) && F a7) -> ((((a2 U a7) && G ! a7 && G (a1 -> G ! a7)) || (F a5 && F (a7 && F a8) && G (a9 && (! a1 -> (! a1 U (! a1 && a6)))) && G (a5 -> G a3) && G ! a8 && G ! a4 && G (a5 -> G ! a1))) -> (((a1 U a5) && F a6) -> (((a7 U a6) && F a2) -> (! a2 U a6)))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_52() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a8) -> (((a4 U a8) && (F a5) && (G (a6 && ((! a7) -> ((! a7) U (a5 && (! a7)))))) && (F a7)) -> ((((a2 U a7) && (G (! a7)) && (G (a1 -> (G (! a7))))) || ((F (a7 && (F a8))) && (G (a9 && ((! a1) -> ((! a1) U ((! a1) && a6))))) && (G (a5 -> (G a3))) && (G (! a8)) && (G (! a4)) && (G (a9 -> (G (! a1)))) && (F a1))) -> (((F a5) && (a5 U a1)) -> (((F a7) && (a9 U a5)) -> ((! a7) U a6))))))";
    let f_no_par = "F a8 -> (((a4 U a8) && F a5 && G (a6 && (! a7 -> (! a7 U (a5 && ! a7)))) && F a7) -> ((((a2 U a7) && G ! a7 && G (a1 -> G ! a7)) || (F (a7 && F a8) && G (a9 && (! a1 -> (! a1 U (! a1 && a6)))) && G (a5 -> G a3) && G ! a8 && G ! a4 && G (a9 -> G ! a1) && F a1)) -> ((F a5 && (a5 U a1)) -> ((F a7 && (a9 U a5)) -> (! a7 U a6)))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_53() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10", "a11",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a8) -> (((a4 U a8) && (F a5) && (G (a6 && ((! a7) -> ((! a7) U (a5 && (! a7)))))) && (F a7)) -> ((((a2 U a7) && (G (! a7)) && (G (a1 -> (G (! a7))))) || ((F a5) && (F (a7 && (F a8))) && (G (a9 && ((! a1) -> ((! a1) U ((! a1) && a6))))) && (G (a5 -> (G a3))) && (G (! a8)) && (G (! a11)) && (G (a1 -> (G (! a2)))))) -> (((a9 U a5) && (F a9)) -> (((F a5) && (a1 U a9)) -> ((! a5) U a7))))))";
    let f_no_par = "F a8 -> (((a4 U a8) && F a5 && G (a6 && (! a7 -> (! a7 U (a5 && ! a7)))) && F a7) -> ((((a2 U a7) && G ! a7 && G (a1 -> G ! a7)) || (F a5 && F (a7 && F a8) && G (a9 && (! a1 -> (! a1 U (! a1 && a6)))) && G (a5 -> G a3) && G ! a8 && G ! a11 && G (a1 -> G ! a2))) -> (((a9 U a5) && F a9) -> ((F a5 && (a1 U a9)) -> (! a5 U a7)))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_54() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10", "a11", "a12",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a8) -> (((a4 U a8) && (F a5) && (G (a12 && ((! a1) -> ((! a1) U ((! a1) && a6))))) && (F a7)) -> ((((a11 U a7) && (G (a2 -> (G (! a5)))) && (G (! a7))) || ((G (! a5)) && (F (a7 && (F a12))) && (G (a1 && ((! a7) -> ((! a7) U ((! a7) && a8))))) && (G (a1 -> (G a9))) && (G (! a6)) && (G (a11 -> (G (! a7)))) && (F a2))) -> (((a4 U a2) && (F a9)) -> (((F a5) && (a1 U a9)) -> ((! a5) U a1))))))";
    let f_no_par = "F a8 -> (((a4 U a8) && F a5 && G (a12 && (! a1 -> (! a1 U (! a1 && a6)))) && F a7) -> ((((a11 U a7) && G (a2 -> G ! a5) && G ! a7) || (G ! a5 && F (a7 && F a12) && G (a1 && (! a7 -> (! a7 U (! a7 && a8)))) && G (a1 -> G a9) && G ! a6 && G (a11 -> G ! a7) && F a2)) -> (((a4 U a2) && F a9) -> ((F a5 && (a1 U a9)) -> (! a5 U a1)))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_55() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10", "a11", "a12", "a13",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a8) -> (((a4 U a8) && (F a5) && (G (a12 && ((! a1) -> ((! a1) U ((! a1) && a6))))) && (F a7)) -> ((((a11 U a7) && (G (a2 -> (G (! a5)))) && (G (! a7))) || ((F a7) && (F (a7 && (F a13))) && (G (a13 && ((! a1) -> ((! a1) U ((! a1) && a7))))) && (G (a13 -> (G a11))) && (G (! a8)) && (G (! a1)) && (G (a5 -> (G (! a3)))))) -> (((a8 U a7) && (F a11)) -> (((a12 U a11) && (F a4)) -> ((! a4) U a2))))))";
    let f_no_par = "F a8 -> (((a4 U a8) && F a5 && G (a12 && (! a1 -> (! a1 U (! a1 && a6)))) && F a7) -> ((((a11 U a7) && G (a2 -> G ! a5) && G ! a7) || (F a7 && F (a7 && F a13) && G (a13 && (! a1 -> (! a1 U (! a1 && a7)))) && G (a13 -> G a11) && G ! a8 && G ! a1 && G (a5 -> G ! a3))) -> (((a8 U a7) && F a11) -> (((a12 U a11) && F a4) -> (! a4 U a2)))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_56() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a2) -> (((F a2) && (a1 U a2) && (G (a1 && ((! a2) -> ((! a2) U (a0 && (! a2)))))) && (F a1)) -> ((((a0 U a1) && (G (! a1)) && (G (a0 -> (G (! a1))))) || ((F a1) && (F (a1 && (F a0))) && (G (a1 && ((! a0) -> ((! a0) U ((! a0) && a2))))) && (G (a1 -> (G a2))) && (G (! a2)) && (G (! a0)) && (G (a1 -> (G (! a2)))))) -> (((F a2) && (a0 U a1)) -> (((a1 U a2) && (F a0)) -> ((! a0) U a1))))))";
    let f_no_par = "F a2 -> ((F a2 && (a1 U a2) && G (a1 && (! a2 -> (! a2 U (a0 && ! a2)))) && F a1) -> ((((a0 U a1) && G ! a1 && G (a0 -> G ! a1)) || (F a1 && F (a1 && F a0) && G (a1 && (! a0 -> (! a0 U (! a0 && a2)))) && G (a1 -> G a2) && G ! a2 && G ! a0 && G (a1 -> G ! a2))) -> ((F a2 && (a0 U a1)) -> (((a1 U a2) && F a0) -> (! a0 U a1)))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_57() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a3) -> (((a2 U a3) && (F a2) && (G (a0 && ((! a1) -> ((! a1) U ((! a1) && a2))))) && (F a0)) -> ((((a2 U a0) && (G (a1 -> (G (! a3)))) && (G (! a0))) || ((F a0) && (G (! a3)) && (F (a0 && (F a1))) && (G (a0 && ((! a1) -> ((! a1) U ((! a1) && a3))))) && (G (a3 -> (G a2))) && (G (! a2)) && (G (a2 -> (G (! a0)))))) -> (((a2 U a0) && (F a1)) -> (((F a3) && (a2 U a1)) -> ((! a3) U a0))))))";
    let f_no_par = "F a3 -> (((a2 U a3) && F a2 && G (a0 && (! a1 -> (! a1 U (! a1 && a2)))) && F a0) -> ((((a2 U a0) && G (a1 -> G ! a3) && G ! a0) || (F a0 && G ! a3 && F (a0 && F a1) && G (a0 && (! a1 -> (! a1 U (! a1 && a3)))) && G (a3 -> G a2) && G ! a2 && G (a2 -> G ! a0))) -> (((a2 U a0) && F a1) -> ((F a3 && (a2 U a1)) -> (! a3 U a0)))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_58() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a4) -> (((F a4) && (a2 U a4) && (F a0) && (G (a3 && ((! a2) -> ((! a2) U (a0 && (! a2))))))) -> ((((a3 U a4) && (G (a3 -> (G (! a0)))) && (G (! a4))) || ((F a4) && (G (! a0)) && (F (a4 && (F a0))) && (G (a3 && ((! a1) -> ((! a1) U ((! a1) && a4))))) && (G (a3 -> (G a4))) && (G (! a2)) && (G (a2 -> (G (! a0)))))) -> (((a3 U a4) && (F a3)) -> (((a1 U a3) && (F a2)) -> ((! a2) U a4))))))";
    let f_no_par = "F a4 -> ((F a4 && (a2 U a4) && F a0 && G (a3 && (! a2 -> (! a2 U (a0 && ! a2))))) -> ((((a3 U a4) && G (a3 -> G ! a0) && G ! a4) || (F a4 && G ! a0 && F (a4 && F a0) && G (a3 && (! a1 -> (! a1 U (! a1 && a4)))) && G (a3 -> G a4) && G ! a2 && G (a2 -> G ! a0))) -> (((a3 U a4) && F a3) -> (((a1 U a3) && F a2) -> (! a2 U a4)))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_59() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a4) -> (((a2 U a4) && (F a2) && (G (a3 && ((! a5) -> ((! a5) U (a2 && (! a5)))))) && (F a3)) -> ((((a1 U a3) && (G (! a3)) && (G (a0 -> (G (! a3))))) || ((F a2) && (F (a3 && (F a4))) && (G (a4 && ((! a0) -> ((! a0) U ((! a0) && a3))))) && (G (a2 -> (G a1))) && (G (! a4)) && (G (! a5)) && (G (a0 -> (G (! a1)))))) -> (((F a4) && (a4 U a2)) -> (((F a2) && (a0 U a4)) -> ((! a2) U a3))))))";
    let f_no_par = "F a4 -> (((a2 U a4) && F a2 && G (a3 && (! a5 -> (! a5 U (a2 && ! a5)))) && F a3) -> ((((a1 U a3) && G ! a3 && G (a0 -> G ! a3)) || (F a2 && F (a3 && F a4) && G (a4 && (! a0 -> (! a0 U (! a0 && a3)))) && G (a2 -> G a1) && G ! a4 && G ! a5 && G (a0 -> G ! a1))) -> ((F a4 && (a4 U a2)) -> ((F a2 && (a0 U a4)) -> (! a2 U a3)))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_60() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a4) -> (((a2 U a4) && (F a2) && (G (a6 && ((! a0) -> ((! a0) U ((! a0) && a3))))) && (F a3)) -> ((((a5 U a3) && (G (a1 -> (G (! a2)))) && (G (! a3))) || ((G (a6 && ((! a0) -> ((! a0) U ((! a0) && a3))))) && (F a3) && (F (a3 && (F a6))) && (G (a6 -> (G a5))) && (G (! a4)) && (G (! a0)) && (G (a2 -> (G (! a1)))))) -> (((a4 U a3) && (F a5)) -> (((F a2) && (a6 U a5)) -> ((! a2) U a1))))))";
    let f_no_par = "F a4 -> (((a2 U a4) && F a2 && G (a6 && (! a0 -> (! a0 U (! a0 && a3)))) && F a3) -> ((((a5 U a3) && G (a1 -> G ! a2) && G ! a3) || (G (a6 && (! a0 -> (! a0 U (! a0 && a3)))) && F a3 && F (a3 && F a6) && G (a6 -> G a5) && G ! a4 && G ! a0 && G (a2 -> G ! a1))) -> (((a4 U a3) && F a5) -> ((F a2 && (a6 U a5)) -> (! a2 U a1)))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_61() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a7) -> (((a4 U a7) && (F a5) && (G (a1 && ((! a3) -> ((! a3) U ((! a3) && a5))))) && (F a2)) -> ((((a7 U a2) && (G (a3 -> (G (! a2)))) && (G (! a7))) || ((G (a1 && ((! a3) -> ((! a3) U ((! a3) && a5))))) && (G (! a7)) && (F (a7 && (F a6))) && (G (a3 -> (G a1))) && (G (! a3)) && (G (a1 -> (G (! a4)))) && (F a4))) -> (((a1 U a4) && (F a0)) -> (((F a5) && (a5 U a0)) -> ((! a5) U a3))))))";
    let f_no_par = "F a7 -> (((a4 U a7) && F a5 && G (a1 && (! a3 -> (! a3 U (! a3 && a5)))) && F a2) -> ((((a7 U a2) && G (a3 -> G ! a2) && G ! a7) || (G (a1 && (! a3 -> (! a3 U (! a3 && a5)))) && G ! a7 && F (a7 && F a6) && G (a3 -> G a1) && G ! a3 && G (a1 -> G ! a4) && F a4)) -> (((a1 U a4) && F a0) -> ((F a5 && (a5 U a0)) -> (! a5 U a3)))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_62() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a5) -> (((a4 U a5) && (F a1) && (G (a7 && ((! a5) -> ((! a5) U (a1 && (! a5)))))) && (F a8)) -> ((((a7 U a8) && (G (a7 -> (G (! a1)))) && (G (! a8))) || ((G (! a1)) && (F (a8 && (F a1))) && (G (a6 && ((! a3) -> ((! a3) U (a2 && (! a3)))))) && (G (a7 -> (G a8))) && (G (! a4)) && (G (a5 -> (G (! a1)))) && (F a6))) -> (((F a6) && (a7 U a6)) -> (((a2 U a6) && (F a4)) -> ((! a4) U a5))))))";
    let f_no_par = "F a5 -> (((a4 U a5) && F a1 && G (a7 && (! a5 -> (! a5 U (a1 && ! a5)))) && F a8) -> ((((a7 U a8) && G (a7 -> G ! a1) && G ! a8) || (G ! a1 && F (a8 && F a1) && G (a6 && (! a3 -> (! a3 U (a2 && ! a3)))) && G (a7 -> G a8) && G ! a4 && G (a5 -> G ! a1) && F a6)) -> ((F a6 && (a7 U a6)) -> (((a2 U a6) && F a4) -> (! a4 U a5)))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_63() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren =
        "((F a8) -> ((a3 U a8) && (F a2) && (G (a0 && ((! a9) -> ((! a9) U (a1 && (! a9))))))))";
    let f_no_par = "F a8 -> ((a3 U a8) && F a2 && G (a0 && (! a9 -> (! a9 U (a1 && ! a9)))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_64() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren =
        "((F a8) -> ((a3 U a8) && (F a2) && (G (a8 && ((! a0) -> ((! a0) U ((! a0) && a9)))))))";
    let f_no_par = "F a8 -> ((a3 U a8) && F a2 && G (a8 && (! a0 -> (! a0 U (! a0 && a9)))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_65() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10", "a11",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren =
        "((F a10) -> ((a3 U a10) && (F a8) && (G (a9 && ((! a8) -> ((! a8) U (a0 && (! a8))))))))";
    let f_no_par = "F a10 -> ((a3 U a10) && F a8 && G (a9 && (! a8 -> (! a8 U (a0 && ! a8)))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_66() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10", "a11", "a12",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren =
        "((F a10) -> ((a3 U a10) && (F a8) && (G (a9 && ((! a8) -> ((! a8) U (a0 && (! a8))))))))";
    let f_no_par = "F a10 -> ((a3 U a10) && F a8 && G (a9 && (! a8 -> (! a8 U (a0 && ! a8)))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_67() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10", "a11", "a12", "a13",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren =
        "((F a3) -> ((a13 U a3) && (F a10) && (G (a2 && ((! a9) -> ((! a9) U (a8 && (! a9))))))))";
    let f_no_par = "F a3 -> ((a13 U a3) && F a10 && G (a2 && (! a9 -> (! a9 U (a8 && ! a9)))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_68() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren =
        "((F a2) -> ((F a2) && (a0 U a2) && (G (a0 && ((! a2) -> ((! a2) U (a1 && (! a2))))))))";
    let f_no_par = "F a2 -> (F a2 && (a0 U a2) && G (a0 && (! a2 -> (! a2 U (a1 && ! a2)))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_69() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren =
        "((F a2) -> ((a1 U a2) && (F a1) && (G (a0 && ((! a3) -> ((! a3) U (a2 && (! a3))))))))";
    let f_no_par = "F a2 -> ((a1 U a2) && F a1 && G (a0 && (! a3 -> (! a3 U (a2 && ! a3)))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_70() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren =
        "((F a4) -> ((F a4) && (a1 U a4) && (G (a0 && ((! a4) -> ((! a4) U (a1 && (! a4))))))))";
    let f_no_par = "F a4 -> (F a4 && (a1 U a4) && G (a0 && (! a4 -> (! a4 U (a1 && ! a4)))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_71() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren =
        "((F a4) -> ((a1 U a4) && (F a1) && (G (a4 && ((! a0) -> ((! a0) U ((! a0) && a5)))))))";
    let f_no_par = "F a4 -> ((a1 U a4) && F a1 && G (a4 && (! a0 -> (! a0 U (! a0 && a5)))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_72() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren =
        "((F a1) -> ((a6 U a1) && (F a5) && (G (a1 && ((! a4) -> ((! a4) U ((! a4) && a5)))))))";
    let f_no_par = "F a1 -> ((a6 U a1) && F a5 && G (a1 && (! a4 -> (! a4 U (! a4 && a5)))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_73() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren =
        "((F a5) -> ((a3 U a5) && (F a2) && (G (a0 && ((! a7) -> ((! a7) U (a6 && (! a7))))))))";
    let f_no_par = "F a5 -> ((a3 U a5) && F a2 && G (a0 && (! a7 -> (! a7 U (a6 && ! a7)))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_74() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren =
        "((F a2) -> ((a3 U a2) && (F a8) && (G (a0 && ((! a1) -> ((! a1) U ((! a1) && a6)))))))";
    let f_no_par = "F a2 -> ((a3 U a2) && F a8 && G (a0 && (! a1 -> (! a1 U (! a1 && a6)))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_75() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a8) -> (((a3 U a8) && (F a2) && (G (a0 && ((! a9) -> ((! a9) U (a1 && (! a9)))))) && (F a0)) -> (a5 U a0)))";
    let f_no_par = "F a8 -> (((a3 U a8) && F a2 && G (a0 && (! a9 -> (! a9 U (a1 && ! a9)))) && F a0) -> (a5 U a0))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_76() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a8) -> (((a3 U a8) && (F a2) && (G (a8 && ((! a0) -> ((! a0) U ((! a0) && a9))))) && (F a5)) -> (a1 U a5)))";
    let f_no_par = "F a8 -> (((a3 U a8) && F a2 && G (a8 && (! a0 -> (! a0 U (! a0 && a9)))) && F a5) -> (a1 U a5))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_77() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10", "a11",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a10) -> (((a3 U a10) && (F a8) && (G (a9 && ((! a8) -> ((! a8) U (a0 && (! a8)))))) && (F a1)) -> (a0 U a1)))";
    let f_no_par = "F a10 -> (((a3 U a10) && F a8 && G (a9 && (! a8 -> (! a8 U (a0 && ! a8)))) && F a1) -> (a0 U a1))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_78() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10", "a11", "a12",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a10) -> (((a3 U a10) && (F a8) && (G (a9 && ((! a8) -> ((! a8) U (a0 && (! a8)))))) && (F a1)) -> (a0 U a1)))";
    let f_no_par = "F a10 -> (((a3 U a10) && F a8 && G (a9 && (! a8 -> (! a8 U (a0 && ! a8)))) && F a1) -> (a0 U a1))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_79() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10", "a11", "a12", "a13",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a10) -> (((a3 U a10) && (F a8) && (G (a9 && ((! a8) -> ((! a8) U (a0 && (! a8)))))) && (F a1)) -> (a0 U a1)))";
    let f_no_par = "F a10 -> (((a3 U a10) && F a8 && G (a9 && (! a8 -> (! a8 U (a0 && ! a8)))) && F a1) -> (a0 U a1))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_80() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a2) -> (((F a2) && (a0 U a2) && (G (a0 && ((! a2) -> ((! a2) U (a1 && (! a2)))))) && (F a0)) -> (a2 U a0)))";
    let f_no_par = "F a2 -> ((F a2 && (a0 U a2) && G (a0 && (! a2 -> (! a2 U (a1 && ! a2)))) && F a0) -> (a2 U a0))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_81() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a2) -> (((a1 U a2) && (F a1) && (G (a0 && ((! a3) -> ((! a3) U (a2 && (! a3)))))) && (F a3)) -> (a2 U a3)))";
    let f_no_par = "F a2 -> (((a1 U a2) && F a1 && G (a0 && (! a3 -> (! a3 U (a2 && ! a3)))) && F a3) -> (a2 U a3))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_82() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a4) -> (((F a4) && (a1 U a4) && (G (a0 && ((! a4) -> ((! a4) U (a1 && (! a4))))))) -> (a0 U a4)))";
    let f_no_par =
        "F a4 -> ((F a4 && (a1 U a4) && G (a0 && (! a4 -> (! a4 U (a1 && ! a4))))) -> (a0 U a4))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_83() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a4) -> (((a1 U a4) && (F a1) && (G (a4 && ((! a0) -> ((! a0) U ((! a0) && a5))))) && (F a2)) -> (a0 U a2)))";
    let f_no_par = "F a4 -> (((a1 U a4) && F a1 && G (a4 && (! a0 -> (! a0 U (! a0 && a5)))) && F a2) -> (a0 U a2))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_84() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a5) -> (((a1 U a5) && (F a4) && (G (a4 && ((! a6) -> ((! a6) U (a0 && (! a6)))))) && (F a6)) -> (a0 U a6)))";
    let f_no_par = "F a5 -> (((a1 U a5) && F a4 && G (a4 && (! a6 -> (! a6 U (a0 && ! a6)))) && F a6) -> (a0 U a6))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_85() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a5) -> (((a3 U a5) && (F a2) && (G (a0 && ((! a7) -> ((! a7) U (a6 && (! a7)))))) && (F a7)) -> (a5 U a7)))";
    let f_no_par = "F a5 -> (((a3 U a5) && F a2 && G (a0 && (! a7 -> (! a7 U (a6 && ! a7)))) && F a7) -> (a5 U a7))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_86() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a2) -> (((a3 U a2) && (F a8) && (G (a0 && ((! a1) -> ((! a1) U ((! a1) && a6))))) && (F a0)) -> (a5 U a0)))";
    let f_no_par = "F a2 -> (((a3 U a2) && F a8 && G (a0 && (! a1 -> (! a1 U (! a1 && a6)))) && F a0) -> (a5 U a0))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_87() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a2) -> (((a8 U a2) && (F a9) && (G (a0 && ((! a9) -> ((! a9) U (a1 && (! a9)))))) && (F a0)) -> ((a5 U a0) && (G (a2 -> (G (! a0)))))))";
    let f_no_par = "F a2 -> (((a8 U a2) && F a9 && G (a0 && (! a9 -> (! a9 U (a1 && ! a9)))) && F a0) -> ((a5 U a0) && G (a2 -> G ! a0)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_88() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a2) -> (((a8 U a2) && (F a9) && (G (a0 && ((! a10) -> ((! a10) U (a1 && (! a10)))))) && (F a0)) -> ((a5 U a0) && (G (a2 -> (G (! a0)))))))";
    let f_no_par = "F a2 -> (((a8 U a2) && F a9 && G (a0 && (! a10 -> (! a10 U (a1 && ! a10)))) && F a0) -> ((a5 U a0) && G (a2 -> G ! a0)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_89() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10", "a11",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a2) -> (((a8 U a2) && (F a9) && (G (a0 && ((! a11) -> ((! a11) U (a1 && (! a11)))))) && (F a10)) -> ((a5 U a10) && (G (a11 -> (G (! a0)))))))";
    let f_no_par = "F a2 -> (((a8 U a2) && F a9 && G (a0 && (! a11 -> (! a11 U (a1 && ! a11)))) && F a10) -> ((a5 U a10) && G (a11 -> G ! a0)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_90() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10", "a11", "a12",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a2) -> (((a8 U a2) && (F a9) && (G (a12 && ((! a0) -> ((! a0) U ((! a0) && a11))))) && (F a5)) -> ((a1 U a5) && (G (a0 -> (G (! a10)))))))";
    let f_no_par = "F a2 -> (((a8 U a2) && F a9 && G (a12 && (! a0 -> (! a0 U (! a0 && a11)))) && F a5) -> ((a1 U a5) && G (a0 -> G ! a10)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_91() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10", "a11", "a12", "a13",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a2) -> (((a8 U a2) && (F a9) && (G (a12 && ((! a0) -> ((! a0) U ((! a0) && a13))))) && (F a12)) -> ((a1 U a12) && (G (a10 -> (G (! a5)))))))";
    let f_no_par = "F a2 -> (((a8 U a2) && F a9 && G (a12 && (! a0 -> (! a0 U (! a0 && a13)))) && F a12) -> ((a1 U a12) && G (a10 -> G ! a5)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_92() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a0) -> (((F a0) && (a2 U a0) && (F a2) && (G (a0 && ((! a2) -> ((! a2) U (a1 && (! a2))))))) -> ((a2 U a0) && (G (a2 -> (G (! a0)))))))";
    let f_no_par = "F a0 -> ((F a0 && (a2 U a0) && F a2 && G (a0 && (! a2 -> (! a2 U (a1 && ! a2))))) -> ((a2 U a0) && G (a2 -> G ! a0)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_93() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a2) -> (((a1 U a2) && (F a0) && (G (a0 && ((! a1) -> ((! a1) U ((! a1) && a3))))) && (F a3)) -> ((a0 U a3) && (G (a3 -> (G (! a2)))))))";
    let f_no_par = "F a2 -> (((a1 U a2) && F a0 && G (a0 && (! a1 -> (! a1 U (! a1 && a3)))) && F a3) -> ((a0 U a3) && G (a3 -> G ! a2)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_94() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a1) -> (((a4 U a1) && (F a4) && (G (a0 && ((! a4) -> ((! a4) U (a1 && (! a4))))))) -> ((a0 U a4) && (G (a2 -> (G (! a1)))))))";
    let f_no_par = "F a1 -> (((a4 U a1) && F a4 && G (a0 && (! a4 -> (! a4 U (a1 && ! a4))))) -> ((a0 U a4) && G (a2 -> G ! a1)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_95() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a1) -> (((a4 U a1) && (F a4) && (G (a0 && ((! a5) -> ((! a5) U (a4 && (! a5)))))) && (F a0)) -> ((a2 U a0) && (G (a1 -> (G (! a0)))))))";
    let f_no_par = "F a1 -> (((a4 U a1) && F a4 && G (a0 && (! a5 -> (! a5 U (a4 && ! a5)))) && F a0) -> ((a2 U a0) && G (a1 -> G ! a0)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_96() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a1) -> (((a4 U a1) && (F a4) && (G (a6 && ((! a0) -> ((! a0) U ((! a0) && a5))))) && (F a2)) -> ((a0 U a2) && (G (a0 -> (G (! a5)))))))";
    let f_no_par = "F a1 -> (((a4 U a1) && F a4 && G (a6 && (! a0 -> (! a0 U (! a0 && a5)))) && F a2) -> ((a0 U a2) && G (a0 -> G ! a5)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_97() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a4) -> (((a2 U a4) && (F a0) && (G (a1 && ((! a6) -> ((! a6) U (a2 && (! a6)))))) && (F a6)) -> ((a0 U a6) && (G (a1 -> (G (! a0)))))))";
    let f_no_par = "F a4 -> (((a2 U a4) && F a0 && G (a1 && (! a6 -> (! a6 U (a2 && ! a6)))) && F a6) -> ((a0 U a6) && G (a1 -> G ! a0)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_98() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a2) -> (((a8 U a2) && (F a8) && (G (a0 && ((! a1) -> ((! a1) U ((! a1) && a6))))) && (F a0)) -> ((a5 U a0) && (G (a2 -> (G (! a0)))))))";
    let f_no_par = "F a2 -> (((a8 U a2) && F a8 && G (a0 && (! a1 -> (! a1 U (! a1 && a6)))) && F a0) -> ((a5 U a0) && G (a2 -> G ! a0)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_99() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a8) -> (((a9 U a8) && (F a0) && (G (a1 && ((! a5) -> ((! a5) U (a0 && (! a5)))))) && (F a2)) -> ((F (a8 && (F a5))) || ((a0 U a2) && (G (! a8)) && (G (a4 -> (G (! a8))))))))";
    let f_no_par = "F a8 -> (((a9 U a8) && F a0 && G (a1 && (! a5 -> (! a5 U (a0 && ! a5)))) && F a2) -> (F (a8 && F a5) || ((a0 U a2) && G ! a8 && G (a4 -> G ! a8))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_100() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a8) -> (((a9 U a8) && (F a0) && (G (a1 && ((! a5) -> ((! a5) U (a0 && (! a5)))))) && (F a2)) -> ((F (a8 && (F a5))) || ((a0 U a2) && (G (! a8)) && (G (a4 -> (G (! a8))))))))";
    let f_no_par = "F a8 -> (((a9 U a8) && F a0 && G (a1 && (! a5 -> (! a5 U (a0 && ! a5)))) && F a2) -> (F (a8 && F a5) || ((a0 U a2) && G ! a8 && G (a4 -> G ! a8))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_101() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10", "a11",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a8) -> (((a9 U a8) && (F a0) && (G (a1 && ((! a5) -> ((! a5) U (a0 && (! a5)))))) && (F a2)) -> ((F (a8 && (F a5))) || ((a0 U a2) && (G (! a8)) && (G (a4 -> (G (! a8))))))))";
    let f_no_par = "F a8 -> (((a9 U a8) && F a0 && G (a1 && (! a5 -> (! a5 U (a0 && ! a5)))) && F a2) -> (F (a8 && F a5) || ((a0 U a2) && G ! a8 && G (a4 -> G ! a8))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_102() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10", "a11", "a12",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a8) -> (((a9 U a8) && (F a12) && (G (a0 && ((! a1) -> ((! a1) U ((! a1) && a5))))) && (F a0)) -> ((F (a8 && (F a4))) || ((a10 U a0) && (G (a2 -> (G (! a0)))) && (G (! a8))))))";
    let f_no_par = "F a8 -> (((a9 U a8) && F a12 && G (a0 && (! a1 -> (! a1 U (! a1 && a5)))) && F a0) -> (F (a8 && F a4) || ((a10 U a0) && G (a2 -> G ! a0) && G ! a8)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_103() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10", "a11", "a12", "a13",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a8) -> (((a9 U a8) && (F a12) && (G (a0 && ((! a1) -> ((! a1) U ((! a1) && a5))))) && (F a0)) -> ((F (a2 && (F a8))) || ((a10 U a0) && (G (a0 -> (G (! a13)))) && (G (! a2))))))";
    let f_no_par = "F a8 -> (((a9 U a8) && F a12 && G (a0 && (! a1 -> (! a1 U (! a1 && a5)))) && F a0) -> (F (a2 && F a8) || ((a10 U a0) && G (a0 -> G ! a13) && G ! a2)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_104() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a0) -> (((F a0) && (a2 U a0) && (G (a1 && ((! a0) -> ((! a0) U ((! a0) && a2))))) && (F a1)) -> ((F (a2 && (F a0))) || ((a0 U a1) && (G (! a2)) && (G (a1 -> (G (! a2))))))))";
    let f_no_par = "F a0 -> ((F a0 && (a2 U a0) && G (a1 && (! a0 -> (! a0 U (! a0 && a2)))) && F a1) -> (F (a2 && F a0) || ((a0 U a1) && G ! a2 && G (a1 -> G ! a2))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_105() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a3) -> (((F a3) && (a0 U a3) && (F a0) && (G (a0 && ((! a3) -> ((! a3) U (a2 && (! a3))))))) -> ((F (a0 && (F a1))) || ((a2 U a3) && (G (a3 -> (G (! a2)))) && (G (! a0))))))";
    let f_no_par = "F a3 -> ((F a3 && (a0 U a3) && F a0 && G (a0 && (! a3 -> (! a3 U (a2 && ! a3))))) -> (F (a0 && F a1) || ((a2 U a3) && G (a3 -> G ! a2) && G ! a0)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_106() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a0) -> (((F a0) && (a4 U a0) && (G (a2 && ((! a0) -> ((! a0) U ((! a0) && a3))))) && (F a2)) -> ((F (a0 && (F a3))) || ((a1 U a2) && (G (a2 -> (G (! a4)))) && (G (! a0))))))";
    let f_no_par = "F a0 -> ((F a0 && (a4 U a0) && G (a2 && (! a0 -> (! a0 U (! a0 && a3)))) && F a2) -> (F (a0 && F a3) || ((a1 U a2) && G (a2 -> G ! a4) && G ! a0)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_107() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a5) -> (((a4 U a5) && (F a0) && (G (a0 && ((! a2) -> ((! a2) U ((! a2) && a5))))) && (F a1)) -> ((F (a4 && (F a2))) || ((a0 U a1) && (G (! a4)) && (G (a2 -> (G (! a4))))))))";
    let f_no_par = "F a5 -> (((a4 U a5) && F a0 && G (a0 && (! a2 -> (! a2 U (! a2 && a5)))) && F a1) -> (F (a4 && F a2) || ((a0 U a1) && G ! a4 && G (a2 -> G ! a4))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_108() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a6) -> (((F a6) && (a4 U a6) && (G (a0 && ((! a6) -> ((! a6) U (a2 && (! a6)))))) && (F a0)) -> ((F (a1 && (F a4))) || ((a5 U a0) && (G (a0 -> (G (! a6)))) && (G (! a1))))))";
    let f_no_par = "F a6 -> ((F a6 && (a4 U a6) && G (a0 && (! a6 -> (! a6 U (a2 && ! a6)))) && F a0) -> (F (a1 && F a4) || ((a5 U a0) && G (a0 -> G ! a6) && G ! a1)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_109() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a7) -> (((a0 U a7) && (F a1) && (G (a5 && ((! a7) -> ((! a7) U (a0 && (! a7))))))) -> ((F (a5 && (F a7))) || ((a0 U a1) && (G (a7 -> (G (! a4)))) && (G (! a5))))))";
    let f_no_par = "F a7 -> (((a0 U a7) && F a1 && G (a5 && (! a7 -> (! a7 U (a0 && ! a7))))) -> (F (a5 && F a7) || ((a0 U a1) && G (a7 -> G ! a4) && G ! a5)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_110() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a0) -> (((F a0) && (a8 U a0) && (G (a5 && ((! a0) -> ((! a0) U ((! a0) && a6))))) && (F a2)) -> ((F (a8 && (F a5))) || ((a0 U a2) && (G (! a8)) && (G (a4 -> (G (! a8))))))))";
    let f_no_par = "F a0 -> ((F a0 && (a8 U a0) && G (a5 && (! a0 -> (! a0 U (! a0 && a6)))) && F a2) -> (F (a8 && F a5) || ((a0 U a2) && G ! a8 && G (a4 -> G ! a8))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_111() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a9) -> (((a0 U a9) && (F a1) && (G (a0 && ((! a9) -> ((! a9) U (a2 && (! a9)))))) && (F a4)) -> (((a8 U a4) && (G (a5 -> (G (! a8)))) && (G (! a1))) || ((F (a1 && (F a6))) && (G (a7 && ((! a5) -> ((! a5) U (a2 && (! a5))))))))))";
    let f_no_par = "F a9 -> (((a0 U a9) && F a1 && G (a0 && (! a9 -> (! a9 U (a2 && ! a9)))) && F a4) -> (((a8 U a4) && G (a5 -> G ! a8) && G ! a1) || (F (a1 && F a6) && G (a7 && (! a5 -> (! a5 U (a2 && ! a5)))))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_112() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a10) -> (((a0 U a10) && (F a1) && (G (a10 && ((! a0) -> ((! a0) U ((! a0) && a9))))) && (F a8)) -> (((a2 U a8) && (G (a8 -> (G (! a4)))) && (G (! a5))) || ((F (a5 && (F a1))) && (G (a6 && ((! a7) -> ((! a7) U (a5 && (! a7))))))))))";
    let f_no_par = "F a10 -> (((a0 U a10) && F a1 && G (a10 && (! a0 -> (! a0 U (! a0 && a9)))) && F a8) -> (((a2 U a8) && G (a8 -> G ! a4) && G ! a5) || (F (a5 && F a1) && G (a6 && (! a7 -> (! a7 U (a5 && ! a7)))))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_113() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10", "a11",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a11) -> (((a0 U a11) && (F a1) && (G (a10 && ((! a0) -> ((! a0) U ((! a0) && a11))))) && (F a8)) -> (((a2 U a8) && (G (a8 -> (G (! a4)))) && (G (! a5))) || ((F (a5 && (F a1))) && (G (a6 && ((! a7) -> ((! a7) U (a5 && (! a7))))))))))";
    let f_no_par = "F a11 -> (((a0 U a11) && F a1 && G (a10 && (! a0 -> (! a0 U (! a0 && a11)))) && F a8) -> (((a2 U a8) && G (a8 -> G ! a4) && G ! a5) || (F (a5 && F a1) && G (a6 && (! a7 -> (! a7 U (a5 && ! a7)))))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_114() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10", "a11", "a12",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a0) -> (((F a0) && (a12 U a0) && (G (a12 && ((! a5) -> ((! a5) U ((! a5) && a10))))) && (F a12)) -> (((a0 U a12) && (G (a8 -> (G (! a2)))) && (G (! a4))) || ((F (a4 && (F a8))) && (G (a5 && ((! a11) -> ((! a11) U (a1 && (! a11))))))))))";
    let f_no_par = "F a0 -> ((F a0 && (a12 U a0) && G (a12 && (! a5 -> (! a5 U (! a5 && a10)))) && F a12) -> (((a0 U a12) && G (a8 -> G ! a2) && G ! a4) || (F (a4 && F a8) && G (a5 && (! a11 -> (! a11 U (a1 && ! a11)))))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_115() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10", "a11", "a12", "a13",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a0) -> (((F a0) && (a12 U a0) && (G (a12 && ((! a5) -> ((! a5) U ((! a5) && a10)))))) -> (((a13 U a0) && (G (a0 -> (G (! a13)))) && (G (! a2))) || ((F (a2 && (F a8))) && (G (a4 && ((! a8) -> ((! a8) U (a5 && (! a8))))))))))";
    let f_no_par = "F a0 -> ((F a0 && (a12 U a0) && G (a12 && (! a5 -> (! a5 U (! a5 && a10))))) -> (((a13 U a0) && G (a0 -> G ! a13) && G ! a2) || (F (a2 && F a8) && G (a4 && (! a8 -> (! a8 U (a5 && ! a8)))))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_116() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a2) -> (((a0 U a2) && (F a0) && (G (a2 && ((! a0) -> ((! a0) U ((! a0) && a1))))) && (F a1)) -> (((a0 U a1) && (G (! a2)) && (G (a1 -> (G (! a2))))) || ((F (a2 && (F a0))) && (G (a1 && ((! a2) -> ((! a2) U (a0 && (! a2))))))))))";
    let f_no_par = "F a2 -> (((a0 U a2) && F a0 && G (a2 && (! a0 -> (! a0 U (! a0 && a1)))) && F a1) -> (((a0 U a1) && G ! a2 && G (a1 -> G ! a2)) || (F (a2 && F a0) && G (a1 && (! a2 -> (! a2 U (a0 && ! a2)))))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_117() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a3) -> (((F a3) && (a0 U a3) && (F a0) && (G (a0 && ((! a3) -> ((! a3) U (a2 && (! a3))))))) -> (((a2 U a3) && (G (a3 -> (G (! a2)))) && (G (! a0))) || ((F (a0 && (F a1))) && (G (a3 && ((! a1) -> ((! a1) U (a0 && (! a1))))))))))";
    let f_no_par = "F a3 -> ((F a3 && (a0 U a3) && F a0 && G (a0 && (! a3 -> (! a3 U (a2 && ! a3))))) -> (((a2 U a3) && G (a3 -> G ! a2) && G ! a0) || (F (a0 && F a1) && G (a3 && (! a1 -> (! a1 U (a0 && ! a1)))))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_118() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a4) -> (((a0 U a4) && (F a0) && (G (a0 && ((! a4) -> ((! a4) U (a3 && (! a4)))))) && (F a2)) -> (((a4 U a2) && (G (a2 -> (G (! a4)))) && (G (! a0))) || ((F (a0 && (F a3))) && (G (a3 && ((! a2) -> ((! a2) U (a0 && (! a2))))))))))";
    let f_no_par = "F a4 -> (((a0 U a4) && F a0 && G (a0 && (! a4 -> (! a4 U (a3 && ! a4)))) && F a2) -> (((a4 U a2) && G (a2 -> G ! a4) && G ! a0) || (F (a0 && F a3) && G (a3 && (! a2 -> (! a2 U (a0 && ! a2)))))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_119() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a5) -> (((a0 U a5) && (F a0) && (G (a5 && ((! a0) -> ((! a0) U ((! a0) && a4))))) && (F a4)) -> (((a1 U a4) && (G (! a2)) && (G (a4 -> (G (! a2))))) || ((F (a2 && (F a0))) && (G (a3 && ((! a5) -> ((! a5) U (a2 && (! a5))))))))))";
    let f_no_par = "F a5 -> (((a0 U a5) && F a0 && G (a5 && (! a0 -> (! a0 U (! a0 && a4)))) && F a4) -> (((a1 U a4) && G ! a2 && G (a4 -> G ! a2)) || (F (a2 && F a0) && G (a3 && (! a5 -> (! a5 U (a2 && ! a5)))))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_120() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a0) -> (((F a0) && (a6 U a0) && (G (a6 && ((! a2) -> ((! a2) U (a0 && (! a2))))))) -> (((a6 U a0) && (G (a4 -> (G (! a1)))) && (G (! a2))) || ((F (a2 && (F a4))) && (G (a2 && ((! a5) -> ((! a5) U (a0 && (! a5))))))))))";
    let f_no_par = "F a0 -> ((F a0 && (a6 U a0) && G (a6 && (! a2 -> (! a2 U (a0 && ! a2))))) -> (((a6 U a0) && G (a4 -> G ! a1) && G ! a2) || (F (a2 && F a4) && G (a2 && (! a5 -> (! a5 U (a0 && ! a5)))))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_121() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a7) -> (((a0 U a7) && (F a1) && (G (a5 && ((! a7) -> ((! a7) U (a0 && (! a7))))))) -> (((a0 U a1) && (G (a7 -> (G (! a4)))) && (G (! a5))) || ((F (a5 && (F a7))) && (G (a1 && ((! a3) -> ((! a3) U ((! a3) && a5)))))))))";
    let f_no_par = "F a7 -> (((a0 U a7) && F a1 && G (a5 && (! a7 -> (! a7 U (a0 && ! a7))))) -> (((a0 U a1) && G (a7 -> G ! a4) && G ! a5) || (F (a5 && F a7) && G (a1 && (! a3 -> (! a3 U (! a3 && a5)))))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_122() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a8) -> (((a0 U a8) && (F a1) && (G (a0 && ((! a8) -> ((! a8) U (a1 && (! a8)))))) && (F a4)) -> (((a8 U a4) && (G (a5 -> (G (! a8)))) && (G (! a1))) || ((F (a1 && (F a6))) && (G (a7 && ((! a5) -> ((! a5) U (a1 && (! a5))))))))))";
    let f_no_par = "F a8 -> (((a0 U a8) && F a1 && G (a0 && (! a8 -> (! a8 U (a1 && ! a8)))) && F a4) -> (((a8 U a4) && G (a5 -> G ! a8) && G ! a1) || (F (a1 && F a6) && G (a7 && (! a5 -> (! a5 U (a1 && ! a5)))))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_123() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a9) -> (((a0 U a9) && (F a1) && (G (a0 && ((! a9) -> ((! a9) U (a2 && (! a9)))))) && (F a4)) -> (((a8 U a4) && (G (a5 -> (G (! a8)))) && (G (! a1))) || ((F (a1 && (F a6))) && (G (a7 && ((! a5) -> ((! a5) U (a2 && (! a5)))))) && (G (a9 -> (G a7)))))))";
    let f_no_par = "F a9 -> (((a0 U a9) && F a1 && G (a0 && (! a9 -> (! a9 U (a2 && ! a9)))) && F a4) -> (((a8 U a4) && G (a5 -> G ! a8) && G ! a1) || (F (a1 && F a6) && G (a7 && (! a5 -> (! a5 U (a2 && ! a5)))) && G (a9 -> G a7))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_124() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a10) -> (((a0 U a10) && (F a1) && (G (a10 && ((! a0) -> ((! a0) U ((! a0) && a9))))) && (F a8)) -> (((a2 U a8) && (G (a8 -> (G (! a4)))) && (G (! a5))) || ((F (a5 && (F a1))) && (G (a6 && ((! a7) -> ((! a7) U (a5 && (! a7)))))) && (G (a7 -> (G a2)))))))";
    let f_no_par = "F a10 -> (((a0 U a10) && F a1 && G (a10 && (! a0 -> (! a0 U (! a0 && a9)))) && F a8) -> (((a2 U a8) && G (a8 -> G ! a4) && G ! a5) || (F (a5 && F a1) && G (a6 && (! a7 -> (! a7 U (a5 && ! a7)))) && G (a7 -> G a2))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_125() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10", "a11",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a11) -> (((a0 U a11) && (F a1) && (G (a10 && ((! a0) -> ((! a0) U ((! a0) && a11))))) && (F a8)) -> (((a2 U a8) && (G (a8 -> (G (! a4)))) && (G (! a5))) || ((F (a5 && (F a1))) && (G (a6 && ((! a7) -> ((! a7) U (a5 && (! a7)))))) && (G (a7 -> (G a2)))))))";
    let f_no_par = "F a11 -> (((a0 U a11) && F a1 && G (a10 && (! a0 -> (! a0 U (! a0 && a11)))) && F a8) -> (((a2 U a8) && G (a8 -> G ! a4) && G ! a5) || (F (a5 && F a1) && G (a6 && (! a7 -> (! a7 U (a5 && ! a7)))) && G (a7 -> G a2))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_126() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10", "a11", "a12",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a12) -> (((a0 U a12) && (F a1) && (G (a10 && ((! a0) -> ((! a0) U ((! a0) && a11))))) && (F a8)) -> (((a2 U a8) && (G (a8 -> (G (! a4)))) && (G (! a5))) || ((F (a5 && (F a11))) && (G (a12 && ((! a1) -> ((! a1) U ((! a1) && a6))))) && (G (a7 -> (G a11)))))))";
    let f_no_par = "F a12 -> (((a0 U a12) && F a1 && G (a10 && (! a0 -> (! a0 U (! a0 && a11)))) && F a8) -> (((a2 U a8) && G (a8 -> G ! a4) && G ! a5) || (F (a5 && F a11) && G (a12 && (! a1 -> (! a1 U (! a1 && a6)))) && G (a7 -> G a11))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_127() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10", "a11", "a12", "a13",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a13) -> (((a0 U a13) && (F a1) && (G (a5 && ((! a10) -> ((! a10) U (a0 && (! a10)))))) && (F a0)) -> (((a13 U a0) && (G (a8 -> (G (! a2)))) && (G (! a4))) || ((F (a4 && (F a8))) && (G (a5 && ((! a11) -> ((! a11) U (a1 && (! a11)))))) && (G (a11 -> (G a6)))))))";
    let f_no_par = "F a13 -> (((a0 U a13) && F a1 && G (a5 && (! a10 -> (! a10 U (a0 && ! a10)))) && F a0) -> (((a13 U a0) && G (a8 -> G ! a2) && G ! a4) || (F (a4 && F a8) && G (a5 && (! a11 -> (! a11 U (a1 && ! a11)))) && G (a11 -> G a6))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_128() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a2) -> (((a0 U a2) && (F a0) && (G (a2 && ((! a0) -> ((! a0) U ((! a0) && a1))))) && (F a1)) -> (((a0 U a1) && (G (! a2)) && (G (a1 -> (G (! a2))))) || ((F (a2 && (F a0))) && (G (a1 && ((! a2) -> ((! a2) U (a0 && (! a2)))))) && (G (a1 -> (G a0)))))))";
    let f_no_par = "F a2 -> (((a0 U a2) && F a0 && G (a2 && (! a0 -> (! a0 U (! a0 && a1)))) && F a1) -> (((a0 U a1) && G ! a2 && G (a1 -> G ! a2)) || (F (a2 && F a0) && G (a1 && (! a2 -> (! a2 U (a0 && ! a2)))) && G (a1 -> G a0))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_129() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a3) -> (((F a3) && (a0 U a3) && (F a0) && (G (a0 && ((! a3) -> ((! a3) U (a2 && (! a3))))))) -> (((a2 U a3) && (G (a3 -> (G (! a2)))) && (G (! a0))) || ((F (a0 && (F a1))) && (G (a3 && ((! a1) -> ((! a1) U (a0 && (! a1)))))) && (G (a1 -> (G a3)))))))";
    let f_no_par = "F a3 -> ((F a3 && (a0 U a3) && F a0 && G (a0 && (! a3 -> (! a3 U (a2 && ! a3))))) -> (((a2 U a3) && G (a3 -> G ! a2) && G ! a0) || (F (a0 && F a1) && G (a3 && (! a1 -> (! a1 U (a0 && ! a1)))) && G (a1 -> G a3))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_130() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a4) -> (((a0 U a4) && (F a0) && (G (a0 && ((! a4) -> ((! a4) U (a3 && (! a4)))))) && (F a2)) -> (((a4 U a2) && (G (a2 -> (G (! a4)))) && (G (! a0))) || ((F (a0 && (F a3))) && (G (a3 && ((! a2) -> ((! a2) U (a0 && (! a2)))))) && (G (a4 -> (G a3)))))))";
    let f_no_par = "F a4 -> (((a0 U a4) && F a0 && G (a0 && (! a4 -> (! a4 U (a3 && ! a4)))) && F a2) -> (((a4 U a2) && G (a2 -> G ! a4) && G ! a0) || (F (a0 && F a3) && G (a3 && (! a2 -> (! a2 U (a0 && ! a2)))) && G (a4 -> G a3))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_131() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a5) -> (((a0 U a5) && (F a0) && (G (a5 && ((! a0) -> ((! a0) U ((! a0) && a4))))) && (F a4)) -> (((a1 U a4) && (G (! a2)) && (G (a4 -> (G (! a2))))) || ((F (a2 && (F a0))) && (G (a3 && ((! a5) -> ((! a5) U (a2 && (! a5)))))) && (G (a3 -> (G a1)))))))";
    let f_no_par = "F a5 -> (((a0 U a5) && F a0 && G (a5 && (! a0 -> (! a0 U (! a0 && a4)))) && F a4) -> (((a1 U a4) && G ! a2 && G (a4 -> G ! a2)) || (F (a2 && F a0) && G (a3 && (! a5 -> (! a5 U (a2 && ! a5)))) && G (a3 -> G a1))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_132() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a6) -> (((a0 U a6) && (F a0) && (G (a5 && ((! a0) -> ((! a0) U ((! a0) && a6))))) && (F a4)) -> (((a1 U a4) && (G (! a2)) && (G (a4 -> (G (! a2))))) || ((F (a2 && (F a5))) && (G (a6 && ((! a0) -> ((! a0) U ((! a0) && a3))))) && (G (a3 -> (G a5)))))))";
    let f_no_par = "F a6 -> (((a0 U a6) && F a0 && G (a5 && (! a0 -> (! a0 U (! a0 && a6)))) && F a4) -> (((a1 U a4) && G ! a2 && G (a4 -> G ! a2)) || (F (a2 && F a5) && G (a6 && (! a0 -> (! a0 U (! a0 && a3)))) && G (a3 -> G a5))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_133() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a7) -> (((a0 U a7) && (F a1) && (G (a5 && ((! a7) -> ((! a7) U (a0 && (! a7))))))) -> (((a0 U a1) && (G (a7 -> (G (! a4)))) && (G (! a5))) || ((F (a5 && (F a7))) && (G (a1 && ((! a3) -> ((! a3) U ((! a3) && a5))))) && (G (a2 -> (G a7)))))))";
    let f_no_par = "F a7 -> (((a0 U a7) && F a1 && G (a5 && (! a7 -> (! a7 U (a0 && ! a7))))) -> (((a0 U a1) && G (a7 -> G ! a4) && G ! a5) || (F (a5 && F a7) && G (a1 && (! a3 -> (! a3 U (! a3 && a5)))) && G (a2 -> G a7))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_134() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a8) -> (((a0 U a8) && (F a1) && (G (a0 && ((! a8) -> ((! a8) U (a1 && (! a8)))))) && (F a4)) -> (((a8 U a4) && (G (a5 -> (G (! a8)))) && (G (! a1))) || ((F (a1 && (F a6))) && (G (a7 && ((! a5) -> ((! a5) U (a1 && (! a5)))))) && (G (a8 -> (G a7)))))))";
    let f_no_par = "F a8 -> (((a0 U a8) && F a1 && G (a0 && (! a8 -> (! a8 U (a1 && ! a8)))) && F a4) -> (((a8 U a4) && G (a5 -> G ! a8) && G ! a1) || (F (a1 && F a6) && G (a7 && (! a5 -> (! a5 U (a1 && ! a5)))) && G (a8 -> G a7))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_135() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a5) -> (((F a5) && (a1 U a5) && (F a0) && (G (a2 && ((! a8) -> ((! a8) U (a4 && (! a8))))))) -> (((a8 U a5) && (G (a6 -> (G (! a1)))) && (G (! a7))) || ((F (a7 && (F a5))) && (G (a2 && ((! a7) -> ((! a7) U ((! a7) && a8))))) && (G (a7 -> (G a1))) && (G (! a8))))))";
    let f_no_par = "F a5 -> ((F a5 && (a1 U a5) && F a0 && G (a2 && (! a8 -> (! a8 U (a4 && ! a8))))) -> (((a8 U a5) && G (a6 -> G ! a1) && G ! a7) || (F (a7 && F a5) && G (a2 && (! a7 -> (! a7 U (! a7 && a8)))) && G (a7 -> G a1) && G ! a8)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_136() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a5) -> (((a1 U a5) && (F a10) && (G (a0 && ((! a2) -> ((! a2) U ((! a2) && a8))))) && (F a8)) -> (((a4 U a8) && (G (a1 -> (G (! a5)))) && (G (! a6))) || ((F (a6 && (F a7))) && (G (a5 && ((! a2) -> ((! a2) U ((! a2) && a7))))) && (G (a1 -> (G a7))) && (G (! a7))))))";
    let f_no_par = "F a5 -> (((a1 U a5) && F a10 && G (a0 && (! a2 -> (! a2 U (! a2 && a8)))) && F a8) -> (((a4 U a8) && G (a1 -> G ! a5) && G ! a6) || (F (a6 && F a7) && G (a5 && (! a2 -> (! a2 U (! a2 && a7)))) && G (a1 -> G a7) && G ! a7)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_137() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10", "a11",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a5) -> (((a1 U a5) && (F a10) && (G (a0 && ((! a2) -> ((! a2) U ((! a2) && a8))))) && (F a8)) -> (((a4 U a8) && (G (a1 -> (G (! a5)))) && (G (! a6))) || ((F (a6 && (F a7))) && (G (a5 && ((! a2) -> ((! a2) U ((! a2) && a7))))) && (G (a1 -> (G a7))) && (G (! a7))))))";
    let f_no_par = "F a5 -> (((a1 U a5) && F a10 && G (a0 && (! a2 -> (! a2 U (! a2 && a8)))) && F a8) -> (((a4 U a8) && G (a1 -> G ! a5) && G ! a6) || (F (a6 && F a7) && G (a5 && (! a2 -> (! a2 U (! a2 && a7)))) && G (a1 -> G a7) && G ! a7)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_138() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10", "a11", "a12",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a5) -> (((a1 U a5) && (F a10) && (G (a0 && ((! a2) -> ((! a2) U ((! a2) && a8))))) && (F a8)) -> (((a4 U a8) && (G (a11 -> (G (! a5)))) && (G (! a12))) || ((F (a12 && (F a1))) && (G (a6 && ((! a11) -> ((! a11) U (a7 && (! a11)))))) && (G (a2 -> (G a5))) && (G (! a7))))))";
    let f_no_par = "F a5 -> (((a1 U a5) && F a10 && G (a0 && (! a2 -> (! a2 U (! a2 && a8)))) && F a8) -> (((a4 U a8) && G (a11 -> G ! a5) && G ! a12) || (F (a12 && F a1) && G (a6 && (! a11 -> (! a11 U (a7 && ! a11)))) && G (a2 -> G a5) && G ! a7)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_139() {
    let atomic_props: Vec<_> = [
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "a10", "a11", "a12", "a13",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "((F a12) -> (((a1 U a12) && (F a5) && (G (a13 && ((! a0) -> ((! a0) U ((! a0) && a12))))) && (F a8)) -> (((a2 U a8) && (G (a8 -> (G (! a4)))) && (G (! a5))) || ((G (! a5)) && (F (a5 && (F a11))) && (G (a12 && ((! a1) -> ((! a1) U ((! a1) && a6))))) && (G (a7 -> (G a11)))))))";
    let f_no_par = "F a12 -> (((a1 U a12) && F a5 && G (a13 && (! a0 -> (! a0 U (! a0 && a12)))) && F a8) -> (((a2 U a8) && G (a8 -> G ! a4) && G ! a5) || (G ! a5 && F (a5 && F a11) && G (a12 && (! a1 -> (! a1 U (! a1 && a6)))) && G (a7 -> G a11))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_140() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a1) -> (((F a1) && (a0 U a1) && (F a2) && (G (a0 && ((! a2) -> ((! a2) U (a1 && (! a2))))))) -> (((a2 U a1) && (G (a0 -> (G (! a2)))) && (G (! a1))) || ((G (! a1)) && (F (a1 && (F a2))) && (G (a1 && ((! a0) -> ((! a0) U ((! a0) && a2))))) && (G (a0 -> (G a1)))))))";
    let f_no_par = "F a1 -> ((F a1 && (a0 U a1) && F a2 && G (a0 && (! a2 -> (! a2 U (a1 && ! a2))))) -> (((a2 U a1) && G (a0 -> G ! a2) && G ! a1) || (G ! a1 && F (a1 && F a2) && G (a1 && (! a0 -> (! a0 U (! a0 && a2)))) && G (a0 -> G a1))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_141() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a1) -> (((a0 U a1) && (F a0) && (G (a1 && ((! a2) -> ((! a2) U ((! a2) && a3))))) && (F a3)) -> (((a2 U a3) && (G (a1 -> (G (! a0)))) && (G (! a3))) || ((G (! a0)) && (F (a3 && (F a1))) && (G (a1 && ((! a3) -> ((! a3) U (a2 && (! a3)))))) && (G (a1 -> (G a0)))))))";
    let f_no_par = "F a1 -> (((a0 U a1) && F a0 && G (a1 && (! a2 -> (! a2 U (! a2 && a3)))) && F a3) -> (((a2 U a3) && G (a1 -> G ! a0) && G ! a3) || (G ! a0 && F (a3 && F a1) && G (a1 && (! a3 -> (! a3 U (a2 && ! a3)))) && G (a1 -> G a0))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_142() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a2) -> (((a0 U a2) && (F a0) && (G (a1 && ((! a2) -> ((! a2) U ((! a2) && a3)))))) -> (((a2 U a0) && (G (a4 -> (G (! a3)))) && (G (! a2))) || ((G (! a3)) && (F (a2 && (F a1))) && (G (a3 && ((! a4) -> ((! a4) U (a0 && (! a4)))))) && (G (a0 -> (G a3)))))))";
    let f_no_par = "F a2 -> (((a0 U a2) && F a0 && G (a1 && (! a2 -> (! a2 U (! a2 && a3))))) -> (((a2 U a0) && G (a4 -> G ! a3) && G ! a2) || (G ! a3 && F (a2 && F a1) && G (a3 && (! a4 -> (! a4 U (a0 && ! a4)))) && G (a0 -> G a3))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_143() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a2) -> (((F a2) && (a0 U a2) && (F a5) && (G (a0 && ((! a1) -> ((! a1) U ((! a1) && a2)))))) -> (((a4 U a2) && (G (a0 -> (G (! a5)))) && (G (! a3))) || ((G (! a3)) && (F (a3 && (F a5))) && (G (a2 && ((! a1) -> ((! a1) U ((! a1) && a3))))) && (G (a0 -> (G a3)))))))";
    let f_no_par = "F a2 -> ((F a2 && (a0 U a2) && F a5 && G (a0 && (! a1 -> (! a1 U (! a1 && a2))))) -> (((a4 U a2) && G (a0 -> G ! a5) && G ! a3) || (G ! a3 && F (a3 && F a5) && G (a2 && (! a1 -> (! a1 U (! a1 && a3)))) && G (a0 -> G a3))))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_144() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a2) -> (((F a2) && (a0 U a2) && (F a5) && (G (a6 && ((! a0) -> ((! a0) U ((! a0) && a1)))))) -> (((a4 U a2) && (G (a2 -> (G (! a4)))) && (G (! a5))) || ((F (a5 && (F a0))) && (G (a3 && ((! a5) -> ((! a5) U ((! a5) && a6))))) && (G (a1 -> (G a2))) && (G (! a3))))))";
    let f_no_par = "F a2 -> ((F a2 && (a0 U a2) && F a5 && G (a6 && (! a0 -> (! a0 U (! a0 && a1))))) -> (((a4 U a2) && G (a2 -> G ! a4) && G ! a5) || (F (a5 && F a0) && G (a3 && (! a5 -> (! a5 U (! a5 && a6)))) && G (a1 -> G a2) && G ! a3)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_145() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a6) -> (((a1 U a6) && (F a5) && (G (a0 && ((! a6) -> ((! a6) U ((! a6) && a7))))) && (F a4)) -> (((a2 U a4) && (G (a7 -> (G (! a4)))) && (G (! a5))) || ((F (a5 && (F a7))) && (G (a1 && ((! a3) -> ((! a3) U ((! a3) && a5))))) && (G (a2 -> (G a7))) && (G (! a2))))))";
    let f_no_par = "F a6 -> (((a1 U a6) && F a5 && G (a0 && (! a6 -> (! a6 U (! a6 && a7)))) && F a4) -> (((a2 U a4) && G (a7 -> G ! a4) && G ! a5) || (F (a5 && F a7) && G (a1 && (! a3 -> (! a3 U (! a3 && a5)))) && G (a2 -> G a7) && G ! a2)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_146() {
    let atomic_props: Vec<_> = ["a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let f_paren = "((F a5) -> (((a1 U a5) && (F a0) && (G (a2 && ((! a4) -> ((! a4) U ((! a4) && a7))))) && (F a1)) -> (((a5 U a1) && (G (a7 -> (G (! a6)))) && (G (! a5))) || ((F (a5 && (F a2))) && (G (a7 && ((! a8) -> ((! a8) U (a6 && (! a8)))))) && (G (a7 -> (G a1))) && (G (! a8))))))";
    let f_no_par = "F a5 -> (((a1 U a5) && F a0 && G (a2 && (! a4 -> (! a4 U (! a4 && a7)))) && F a1) -> (((a5 U a1) && G (a7 -> G ! a6) && G ! a5) || (F (a5 && F a2) && G (a7 && (! a8 -> (! a8 U (a6 && ! a8)))) && G (a7 -> G a1) && G ! a8)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}

#[test]
fn test_operator_precedence_147() {
    let atomic_props: Vec<_> = [
        "init_counter_0",
        "init_counter_1",
        "init_counter_2",
        "init_counter_3",
        "init_counter_4",
        "init_counter_5",
        "inc",
        "counter_0",
        "counter_1",
        "counter_2",
        "counter_3",
        "counter_4",
        "counter_5",
        "carry_0",
        "carry_1",
        "carry_2",
        "carry_3",
        "carry_4",
        "carry_5",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let f_paren = "(((X[!] counter_0) -> init_counter_0) && (init_counter_0 -> (X counter_0)) && ((X[!] counter_1) -> init_counter_1) && (init_counter_1 -> (X counter_1)) && ((X[!] counter_2) -> init_counter_2) && (init_counter_2 -> (X counter_2)) && ((X[!] counter_3) -> init_counter_3) && (init_counter_3 -> (X counter_3)) && ((X[!] counter_4) -> init_counter_4) && (init_counter_4 -> (X counter_4)) && ((X[!] counter_5) -> init_counter_5) && (init_counter_5 -> (X counter_5)) && ((G ((! inc) -> (X[!] inc))) -> ((X[!] (G (((X[!] carry_0) -> inc) && (inc -> (X carry_0)) && ((X[!] carry_1) -> (counter_0 && (X[!] carry_0))) && ((X[!] carry_2) -> (counter_1 && (X[!] carry_1))) && ((X[!] carry_3) -> (counter_2 && (X[!] carry_2))) && ((X[!] carry_4) -> (counter_3 && (X[!] carry_3))) && ((X[!] carry_5) -> (counter_4 && (X[!] carry_4))) && ((counter_0 && (X carry_0)) -> (X carry_1)) && ((counter_1 && (X carry_1)) -> (X carry_2)) && ((counter_2 && (X carry_2)) -> (X carry_3)) && ((counter_3 && (X carry_3)) -> (X carry_4)) && ((counter_4 && (X carry_4)) -> (X carry_5)) && ((X[!] counter_0) -> (! (counter_0 <-> (X[!] carry_0)))) && ((! (counter_0 <-> (X carry_0))) -> (X counter_0)) && ((X[!] counter_1) -> (! (counter_1 <-> (X[!] carry_1)))) && ((! (counter_1 <-> (X carry_1))) -> (X counter_1)) && ((X[!] counter_2) -> (! (counter_2 <-> (X[!] carry_2)))) && ((! (counter_2 <-> (X carry_2))) -> (X counter_2)) && ((X[!] counter_3) -> (! (counter_3 <-> (X[!] carry_3)))) && ((! (counter_3 <-> (X carry_3))) -> (X counter_3)) && ((X[!] counter_4) -> (! (counter_4 <-> (X[!] carry_4)))) && ((! (counter_4 <-> (X carry_4))) -> (X counter_4)) && ((X[!] counter_5) -> (! (counter_5 <-> (X[!] carry_5)))) && ((! (counter_5 <-> (X carry_5))) -> (X counter_5))))) && (X[!] (F ((! counter_0) && (! counter_1) && (! counter_2) && (! counter_3) && (! counter_4) && (! counter_5)))))))";
    let f_no_par = "(X[!] counter_0 -> init_counter_0) && (init_counter_0 -> X counter_0) && (X[!] counter_1 -> init_counter_1) && (init_counter_1 -> X counter_1) && (X[!] counter_2 -> init_counter_2) && (init_counter_2 -> X counter_2) && (X[!] counter_3 -> init_counter_3) && (init_counter_3 -> X counter_3) && (X[!] counter_4 -> init_counter_4) && (init_counter_4 -> X counter_4) && (X[!] counter_5 -> init_counter_5) && (init_counter_5 -> X counter_5) && (G (! inc -> X[!] inc) -> (X[!] G ((X[!] carry_0 -> inc) && (inc -> X carry_0) && (X[!] carry_1 -> (counter_0 && X[!] carry_0)) && (X[!] carry_2 -> (counter_1 && X[!] carry_1)) && (X[!] carry_3 -> (counter_2 && X[!] carry_2)) && (X[!] carry_4 -> (counter_3 && X[!] carry_3)) && (X[!] carry_5 -> (counter_4 && X[!] carry_4)) && ((counter_0 && X carry_0) -> X carry_1) && ((counter_1 && X carry_1) -> X carry_2) && ((counter_2 && X carry_2) -> X carry_3) && ((counter_3 && X carry_3) -> X carry_4) && ((counter_4 && X carry_4) -> X carry_5) && (X[!] counter_0 -> ! (counter_0 <-> X[!] carry_0)) && (! (counter_0 <-> X carry_0) -> X counter_0) && (X[!] counter_1 -> ! (counter_1 <-> X[!] carry_1)) && (! (counter_1 <-> X carry_1) -> X counter_1) && (X[!] counter_2 -> ! (counter_2 <-> X[!] carry_2)) && (! (counter_2 <-> X carry_2) -> X counter_2) && (X[!] counter_3 -> ! (counter_3 <-> X[!] carry_3)) && (! (counter_3 <-> X carry_3) -> X counter_3) && (X[!] counter_4 -> ! (counter_4 <-> X[!] carry_4)) && (! (counter_4 <-> X carry_4) -> X counter_4) && (X[!] counter_5 -> ! (counter_5 <-> X[!] carry_5)) && (! (counter_5 <-> X carry_5) -> X counter_5)) && X[!] F (! counter_0 && ! counter_1 && ! counter_2 && ! counter_3 && ! counter_4 && ! counter_5)))";
    let fp = parse_ltl_formula(f_paren, &atomic_props).unwrap();
    let fnp = parse_ltl_formula(f_no_par, &atomic_props).unwrap();
    assert_eq!(fp, fnp);
}
