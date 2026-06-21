//! Maps key events to editor actions.
//!
//! [`map_key`] is a pure function from a crossterm [`KeyEvent`] to an optional
//! [`EditorInput`] action, with no TUI or filesystem side effects. The caller
//! applies the action to a [`FileBuffer`](super::file_buffer::FileBuffer),
//! which keeps input mapping testable in isolation (SRP).

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::edit::Move;

/// A resolved editor action produced from a key press.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditorInput {
    /// Insert a literal character at the cursor.
    Insert(char),
    /// Insert a newline at the cursor.
    Newline,
    /// Delete the character before the cursor.
    Backspace,
    /// Move the cursor in a direction.
    Move(Move),
    /// Save the buffer to disk (Ctrl+S).
    Save,
    /// Leave the editor (Esc).
    Quit,
}

/// Translates a key event into an editor action, or `None` if unhandled.
pub fn map_key(key: KeyEvent) -> Option<EditorInput> {
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        return match key.code {
            KeyCode::Char('s') => Some(EditorInput::Save),
            _ => None,
        };
    }
    match key.code {
        KeyCode::Char(c) => Some(EditorInput::Insert(c)),
        KeyCode::Enter => Some(EditorInput::Newline),
        KeyCode::Backspace => Some(EditorInput::Backspace),
        KeyCode::Left => Some(EditorInput::Move(Move::Left)),
        KeyCode::Right => Some(EditorInput::Move(Move::Right)),
        KeyCode::Up => Some(EditorInput::Move(Move::Up)),
        KeyCode::Down => Some(EditorInput::Move(Move::Down)),
        KeyCode::Esc => Some(EditorInput::Quit),
        _ => None,
    }
}
