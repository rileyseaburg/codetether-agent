//! Word-boundary tests: when `@` opens the picker vs. inserts literally.

use super::fixtures::app_with;
use crate::tui::app::symbols::mention::open;

#[test]
fn mid_word_at_inserts_a_literal_character() {
    let mut app = app_with("user", 4);
    assert!(!open(&mut app));
    assert_eq!(app.state.composer.input, "user@");
    assert!(!app.state.symbol_search.active);
}

#[test]
fn word_initial_at_opens_the_picker() {
    let mut app = app_with("", 0);
    assert!(open(&mut app));
    assert_eq!(app.state.composer.input, "@");
    assert!(app.state.symbol_search.active);
}
