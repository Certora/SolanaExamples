use crate::{correct_add, faulty_add};
use {calltrace::cvt_cex_print_u64, cvt::*, cvt_macros::rule, nondet::*};

#[rule]
pub fn rule_faulty_add_performs_addition() {
    let x: u64 = nondet(); // nondet creates a nondeterministic u64.
    let y: u64 = nondet();
    let faulty_add_result = faulty_add(x, y);
    // In case there is a counter example, print the values of x, y, and the
    // result of the addition.
    cvt_cex_print_u64!("Value of x", x);
    cvt_cex_print_u64!("Value of y", y);
    cvt_cex_print_u64!("Value of faulty_add(x, y)", faulty_add_result);
    cvt_assert!(faulty_add_result == x + y);
}

#[rule]
pub fn rule_correct_add_performs_addition() {
    let x: u64 = nondet();
    let y: u64 = nondet();
    let correct_add_result = correct_add(x, y);
    cvt_assert!(correct_add_result == x + y);
}

#[rule]
pub fn rule_with_assumptions() {
    let x: u64 = nondet();
    // Assumptions restrict the possible values for variables.
    cvt_assume!(x < 10);
    cvt_assert!(x < 999);
}

#[rule]
pub fn rule_vacuous() {
    let x: u64 = nondet();
    // The following assumptions are contradicting.
    cvt_assume!(x < 10);
    cvt_assume!(x > 10);
    // This rule is verified only because of the contradicting assumptions.
    // The vacuity check detects this problem.
    cvt_assert!(false);
}

#[rule]
pub fn rule_satisfy() {
    let x: u64 = nondet();
    cvt_assume!(x < 10);
    // This rule passes because satisfy checks that there is *at least* one
    // execution in which the condition is true.
    // This is different from checking that the condition is is true *for all*
    // executions.
    cvt_satisfy!(x < 1);
}
