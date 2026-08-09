//! Confirm and dismiss behavior for an open mention.

use super::fixtures::{app_with, entry};
use crate::tui::app::symbols::mention::{confirm, dismiss, open};

#[test]
fn dismiss_preserves_the_typed_query() {
    let mut app = app_with("", 0);
    assert!(open(&mut app));
    app.state.symbol_search.query = "cap".to_string();
    dismiss(&mut app);
    assert!(!app.state.symbol_search.active);
    assert_eq!(app.state.input, "@cap");
}

#[test]
fn selection_replaces_only_at_character() {
    let mut app = app_with("fix  please", 4);
    open(&mut app);
    app.state.symbol_search.query = "cap".to_string();
    app.state.symbol_search.set_results(vec![entry()]);
    assert!(confirm(&mut app));
    assert_eq!(app.state.input, "fix @capture (`src/pay.rs:12`) please");
    assert!(!app.state.symbol_search.active);
}
