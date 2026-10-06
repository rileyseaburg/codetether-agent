//! A user can always reach slash commands without accepting an answer or tool.

use crate::tui::app::state::App;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub(super) fn allowed(app: &App, key: KeyEvent) -> bool {
    app.state.input.trim_start().starts_with('/')
        || key.code == KeyCode::Char('/')
        || matches!(key.code, KeyCode::PageUp | KeyCode::PageDown)
        || (matches!(key.code, KeyCode::Char('c' | 'q'))
            && key.modifiers.contains(KeyModifiers::CONTROL))
}
