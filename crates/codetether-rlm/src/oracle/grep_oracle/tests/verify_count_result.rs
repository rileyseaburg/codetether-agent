//! Regression for verify_count_result.

use super::*;

#[test]
fn verify_count_result() {
    let oracle = GrepOracle::new(sample_rust_code());
    let answer = "Found 2 async functions";
    let result = oracle.verify(answer, "Count async functions");
    assert_eq!(result, GrepVerification::ExactMatch);
}
