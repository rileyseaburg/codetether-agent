//! Integration tests for the offline browser primitives.

use super::cookie_diff::diff;
use super::cookie_parse::{self, CookieRecord};

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
fn diff_detects_all_three_kinds() {
    let before = vec![
        rec("session", "old"),
        rec("dropped", "x"),
    ];
    let after = vec![
        rec("session", "new"),
        rec("added", "y"),
    ];
    let d = diff(&before, &after);
    assert_eq!(d.added.len(), 1);
    assert_eq!(d.added[0].name, "added");
    assert_eq!(d.removed.len(), 1);
    assert_eq!(d.removed[0].name, "dropped");
    assert_eq!(d.changed.len(), 1);
    assert_eq!(d.changed[0].before, "old");
    assert_eq!(d.changed[0].after, "new");
}

fn rec(name: &str, value: &str) -> CookieRecord {
    CookieRecord { name: name.into(), value: value.into(), attrs: Default::default() }
}
