//! Tests for event preview truncation.

use super::text;

#[test]
fn truncates_only_at_utf8_boundaries() {
    assert_eq!(text("hello", 5), "hello");
    assert_eq!(text("aébc", 2), "a...");
}
