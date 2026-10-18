//! Existing request regression tests.

use super::*;

#[test]
fn detects_explicit_pr_request() {
    assert!(wants_pr("commit this and create a PR"));
}

#[test]
fn rejects_substring_matches() {
    assert!(!wants_pr("create a program"));
    assert!(!wants_pr("open product notes"));
}
