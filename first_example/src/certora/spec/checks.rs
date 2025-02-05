use crate::{correct_add, faulty_add};
use cvlr::prelude::*; // Import common definitions for Certora's formal verification.

#[rule]
pub fn rule_faulty_add_performs_addition() {
    let x: u64 = nondet(); // nondet creates a nondeterministic u64.
    let y: u64 = nondet();
    let faulty_add_result = faulty_add(x, y);
    // In case there is a counter example, print the values of x, y, and the
    // result of the addition.
    clog!(x, y, faulty_add_result);
    cvlr_assert!(faulty_add_result == x + y);
}

#[rule]
pub fn rule_correct_add_performs_addition() {
    let x: u64 = nondet();
    let y: u64 = nondet();
    let correct_add_result = correct_add(x, y);
    cvlr_assert!(correct_add_result == x + y);
}

#[rule]
pub fn rule_with_assumptions() {
    let x: u64 = nondet();
    // Assumptions restrict the possible values for variables.
    cvlr_assume!(x < 10);
    cvlr_assert!(x < 999);
}

#[rule]
pub fn rule_vacuous() {
    let x: u64 = nondet();
    // The following assumptions are contradicting.
    cvlr_assume!(x < 10);
    cvlr_assume!(x > 10);
    // This rule is verified only because of the contradicting assumptions.
    // The vacuity check detects this problem.
    cvlr_assert!(false);
}

#[rule]
pub fn rule_satisfy() {
    let x: u64 = nondet();
    cvlr_assume!(x < 10);
    // This rule passes because satisfy checks that there is *at least* one
    // execution in which the condition is true.
    // This is different from checking that the condition is is true *for all*
    // executions.
    cvlr_satisfy!(x < 1);
}
