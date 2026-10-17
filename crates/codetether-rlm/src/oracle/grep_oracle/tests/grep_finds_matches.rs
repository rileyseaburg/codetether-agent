//! Regression for grep_finds_matches.

use super::*;

#[test]
fn grep_finds_matches() {
    let oracle = GrepOracle::new(sample_rust_code());
    let matches = oracle.grep(r"\basync\s+fn\b").unwrap();
    assert_eq!(matches.len(), 2);
}
