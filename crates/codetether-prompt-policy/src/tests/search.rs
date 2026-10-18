//! Original empty codesearch response classification cases.
use crate::is_codesearch_no_match_output;

#[test]
fn codesearch_no_match_detection_matches_expected_format() {
    assert!(is_codesearch_no_match_output(
        "codesearch",
        true,
        "No matches found for pattern: foo_bar"
    ));
}

#[test]
fn codesearch_no_match_detection_matches_prefixed_output() {
    assert!(is_codesearch_no_match_output(
        "codesearch",
        true,
        "build step 1 codesearch: No matches found for pattern: foo_bar"
    ));
}

#[test]
fn codesearch_no_match_detection_ignores_non_codesearch_tools() {
    assert!(!is_codesearch_no_match_output(
        "grep",
        true,
        "No matches found for pattern: foo_bar"
    ));
}
