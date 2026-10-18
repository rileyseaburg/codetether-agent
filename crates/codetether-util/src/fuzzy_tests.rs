//! Shared subsequence scoring regression test.
use super::fuzzy::fuzzy_score;

#[test]
fn fuzzy_matches_scattered_session_id() {
    assert!(fuzzy_score("a9c", "a3f9-1c2d").is_some());
    assert!(fuzzy_score("x9c", "a3f9-1c2d").is_none());
}
