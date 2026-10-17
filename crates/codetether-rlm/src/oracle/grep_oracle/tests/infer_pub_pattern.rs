//! Regression for infer_pub_pattern.

use super::*;

#[test]
fn infer_pub_pattern() {
    let pattern = GrepOracle::infer_pattern("List all public functions");
    assert_eq!(pattern, Some(r"\bpub\s+fn\b".to_string()));
}
