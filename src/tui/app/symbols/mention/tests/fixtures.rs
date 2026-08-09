//! Shared fixtures for mention tests.

use crate::tui::app::state::{App, AppState};
use crate::tui::symbol_search::SymbolEntry;
use std::path::PathBuf;

/// Build an app whose chat input is `input` with the cursor at `cursor`.
pub(super) fn app_with(input: &str, cursor: usize) -> App {
    let mut app = App {
        state: AppState::default(),
    };
    app.state.input = input.to_string();
    app.state.input_cursor = cursor;
    app
}

/// A representative symbol result.
pub(super) fn entry() -> SymbolEntry {
    SymbolEntry {
        name: "capture".to_string(),
        kind: "Function".to_string(),
        path: PathBuf::from("src/pay.rs"),
        uri: None,
        line: Some(12),
        container: None,
    }
}
