//! Tests for the symbol-search query gate.

use super::{DEBOUNCE, MIN_QUERY_LEN, is_searchable};

#[test]
fn empty_and_single_char_queries_never_hit_the_language_server() {
    assert!(!is_searchable(""));
    assert!(!is_searchable("a"));
    assert_eq!(MIN_QUERY_LEN, 2);
}

#[test]
fn sufficient_queries_are_searchable() {
    assert!(is_searchable("ab"));
}

#[test]
fn debounce_window_is_short_but_nonzero() {
    assert!(DEBOUNCE.as_millis() > 0);
    assert!(DEBOUNCE.as_millis() <= 250);
}

#[test]
fn multibyte_queries_are_counted_by_character_not_byte() {
    // "é" is 2 bytes but 1 char, so it must still be too short.
    assert!(!is_searchable("é"));
    assert!(is_searchable("éé"));
}
