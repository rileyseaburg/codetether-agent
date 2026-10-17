//! Regression for infer_quoted_literal_pattern.

use super::*;

#[test]
fn infer_quoted_literal_pattern() {
    let pattern =
        GrepOracle::infer_pattern("Find all occurrences of 'async fn' in src/rlm/repl.rs");
    assert_eq!(pattern, Some(regex::escape("async fn")));
}
