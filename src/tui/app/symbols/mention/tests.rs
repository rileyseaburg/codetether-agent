use std::path::PathBuf;

use crate::tui::app::state::AppState;
use crate::tui::app::symbols::mention::{confirm, open};
use crate::tui::symbol_search::SymbolEntry;

#[test]
fn selection_replaces_only_at_character() {
    let mut app = crate::tui::app::state::App {
        state: AppState::default(),
    };
    app.state.input = "fix  please".to_string();
    app.state.input_cursor = 4;
    open(&mut app);
    app.state.symbol_search.query = "cap".to_string();
    app.state.symbol_search.set_results(vec![entry()]);
    assert!(confirm(&mut app));
    assert_eq!(app.state.input, "fix @capture (`src/pay.rs:12`) please");
    assert!(!app.state.symbol_search.active);
}

fn entry() -> SymbolEntry {
    SymbolEntry {
        name: "capture".to_string(),
        kind: "Function".to_string(),
        path: PathBuf::from("src/pay.rs"),
        uri: None,
        line: Some(12),
        container: None,
    }
}
