//! Word-boundary tests: when `@` opens the picker vs. inserts literally.

use super::fixtures::app_with;
use crate::tui::app::symbols::mention::{open, starts_mention};

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

#[test]
fn mid_word_at_inserts_a_literal_character() {
    let mut app = app_with("user", 4);
    assert!(!open(&mut app));
    assert_eq!(app.state.input, "user@");
    assert!(!app.state.symbol_search.active);
}

#[test]
fn word_initial_at_opens_the_picker() {
    let mut app = app_with("", 0);
    assert!(open(&mut app));
    assert_eq!(app.state.input, "@");
    assert!(app.state.symbol_search.active);
}
