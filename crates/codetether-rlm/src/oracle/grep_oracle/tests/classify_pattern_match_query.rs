//! Regression for classify_pattern_match_query.

use super::*;

#[test]
fn classify_pattern_match_query() {
    assert_eq!(
        GrepOracle::classify_query("Find all async functions"),
        QueryType::PatternMatch
    );
    assert_eq!(
        GrepOracle::classify_query("Count occurrences of TODO"),
        QueryType::PatternMatch
    );
}
