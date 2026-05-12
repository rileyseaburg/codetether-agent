//! Integration tests for the offline browser primitives.

use super::cookie_diff::diff;
use super::cookie_parse;
use super::tests_helpers::{rec, rec_with};

#[test]
fn parses_value_with_equals() {
    let c = cookie_parse::parse("token=abc=def; HttpOnly").unwrap();
    assert_eq!(c.value, "abc=def");
    assert!(c.attrs.contains_key("httponly"));
}

#[test]
fn parses_minimal_pair() {
    let c = cookie_parse::parse("a=1").unwrap();
    assert!(c.attrs.is_empty());
    assert_eq!(c.name, "a");
    assert_eq!(c.value, "1");
}

#[test]
fn diff_detects_added_removed_changed() {
    let before = vec![rec("session", "old", "/"), rec("dropped", "x", "/")];
    let after = vec![rec("session", "new", "/"), rec("added", "y", "/")];
    let d = diff(&before, &after);
    assert_eq!(d.added.len(), 1);
    assert_eq!(d.added[0].name, "added");
    assert_eq!(d.removed.len(), 1);
    assert_eq!(d.removed[0].name, "dropped");
    assert_eq!(d.changed.len(), 1);
    assert_eq!(d.changed[0].before.value, "old");
    assert_eq!(d.changed[0].after.value, "new");
    assert!(d.duplicates.is_empty());
}

#[test]
fn diff_keys_on_name_domain_path() {
    let before = vec![rec_with("session", "old", "/", Some("a.example"))];
    let after = vec![rec_with("session", "new", "/", Some("b.example"))];
    let d = diff(&before, &after);
    assert_eq!(d.added.len(), 1, "different domain → treated as new cookie");
    assert_eq!(d.removed.len(), 1);
    assert!(d.changed.is_empty());
}

#[test]
fn diff_flags_duplicates() {
    let before = vec![rec("a", "1", "/"), rec("a", "2", "/")];
    let d = diff(&before, &[]);
    assert_eq!(d.duplicates.len(), 1);
    assert!(d.duplicates[0].starts_with("a|"));
}
