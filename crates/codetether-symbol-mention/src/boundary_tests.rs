//! Word-boundary tests for symbol mentions.
use crate::starts_mention;

#[test]
fn at_start_of_input_begins_a_mention() {
    assert!(starts_mention("", 0));
}

#[test]
fn at_after_whitespace_begins_a_mention() {
    assert!(starts_mention("fix ", 4));
}

#[test]
fn at_inside_a_word_is_literal() {
    // Email addresses and decorators must not open the picker.
    assert!(!starts_mention("user", 4));
    assert!(!starts_mention("a@b", 3));
}
