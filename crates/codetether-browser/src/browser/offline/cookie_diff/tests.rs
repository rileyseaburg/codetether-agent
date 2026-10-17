//! Complete-record comparisons and duplicate key indexing.
use super::diff;
use crate::browser::offline::tests_helpers::rec;

#[test]
fn last_duplicate_wins_and_duplicates_are_reported_once() {
    let before = [rec("session", "old", "/"), rec("session", "final", "/")];
    let after = [rec("session", "discard", "/"), rec("session", "final", "/")];
    let delta = diff(&before, &after);
    assert!(delta.added.is_empty() && delta.removed.is_empty() && delta.changed.is_empty());
    assert_eq!(delta.duplicates, ["session||/"]);
}

#[test]
fn attributes_change_a_record_even_when_value_is_unchanged() {
    let before = rec("session", "same", "/");
    let mut after = before.clone();
    after.attrs.insert("httponly".into(), String::new());
    let delta = diff(&[before], &[after]);
    assert_eq!(delta.changed.len(), 1);
    assert!(!delta.changed[0].before.attrs.contains_key("httponly"));
    assert!(delta.changed[0].after.attrs.contains_key("httponly"));
}
