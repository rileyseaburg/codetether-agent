//! Keys that retain command and scroll access during answer review.
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub(super) fn allowed(input: &str, key: KeyEvent) -> bool {
    !input.trim().is_empty()
        || matches!(key.code, KeyCode::Char('/' | 'c' | 'C' | 'r' | 'R'))
        || matches!(key.code, KeyCode::PageUp | KeyCode::PageDown)
        || (matches!(key.code, KeyCode::Char('c' | 'q'))
            && key.modifiers.contains(KeyModifiers::CONTROL))
}
