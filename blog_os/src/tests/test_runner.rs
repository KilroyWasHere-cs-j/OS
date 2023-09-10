use crate::{print, println};

/// This module contains the test runner, which executes all tests and prints their results.

/// Quick test to check if the test runner works.
fn print_test(){
    print!("print test ... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

/// Executes all tests.
pub fn run_all_tests(){
    print_test();
}