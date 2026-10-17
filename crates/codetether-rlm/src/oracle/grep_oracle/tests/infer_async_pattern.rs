//! Regression for infer_async_pattern.

use super::*;

#[test]
fn infer_async_pattern() {
    let pattern = GrepOracle::infer_pattern("Find all async functions");
    assert_eq!(pattern, Some(r"\basync\s+fn\b".to_string()));
}
