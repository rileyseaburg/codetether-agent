//! Tests for fuzzy session filtering in the session picker.

use super::session_fuzzy::fuzzy_score;
use super::session_test_fixtures::{state_with, summary};

#[test]
fn fuzzy_matches_scattered_session_id() {
    assert!(fuzzy_score("a9c", "a3f9-1c2d").is_some());
    assert!(fuzzy_score("x9c", "a3f9-1c2d").is_none());
}

#[test]
fn filtered_sessions_fuzzy_matches_by_id() {
    let state = state_with(
        vec![
            summary("a3f9-1c2d-7e8b", "Fix login bug"),
            summary("b711-44aa-90ff", "Refactor TUI"),
        ],
        "a3f7e",
    );
    let filtered = state.filtered_sessions();
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].1.id, "a3f9-1c2d-7e8b");
}

#[test]
fn filtered_sessions_ranks_tighter_match_first() {
    let sessions = vec![
        summary("x1a2b3", "scattered abc here"),
        summary("abc123", "tight"),
    ];
    let state = state_with(sessions, "abc");
    assert_eq!(state.filtered_sessions()[0].1.id, "abc123");
}

#[test]
fn empty_filter_returns_all_in_order() {
    let state = state_with(vec![summary("one", "1"), summary("two", "2")], "");
    let filtered = state.filtered_sessions();
    assert_eq!((filtered.len(), filtered[0].1.id.as_str()), (2, "one"));
}
