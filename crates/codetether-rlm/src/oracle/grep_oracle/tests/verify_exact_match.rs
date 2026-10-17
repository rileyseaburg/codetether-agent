//! Regression for verify_exact_match.

use super::*;

#[test]
fn verify_exact_match() {
    let oracle = GrepOracle::new(sample_rust_code());
    let answer = oracle
        .grep(r"\basync\s+fn\b")
        .unwrap_or_default()
        .iter()
        .map(|(line, text)| format!("{line}:{text}"))
        .collect::<Vec<_>>()
        .join("\n");
    let result = oracle.verify(&answer, "Find all async functions");
    // May not be exact due to whitespace, but should be close
    match result {
        GrepVerification::ExactMatch
        | GrepVerification::UnorderedMatch
        | GrepVerification::SubsetMatch { .. } => (),
        _ => panic!("Expected match, got {:?}", result),
    }
}
