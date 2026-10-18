//! Key-event repeat policy for actions that should respond while held.

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

/// Return whether a key press/repeat should enter normal dispatch.
/// # Arguments
/// * `key` - Terminal key event including its kind.
/// # Returns
/// True for presses and PageDown repeats; false for other repeats or releases.
/// # Examples
/// ```
/// use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
/// assert!(codetether_tui_navigation::events::dispatchable(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)));
/// ```
pub fn dispatchable(key: KeyEvent) -> bool {
    key.kind == KeyEventKind::Press
        || key.kind == KeyEventKind::Repeat && key.code == KeyCode::PageDown
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::KeyModifiers;

    fn repeated(code: KeyCode) -> KeyEvent {
        KeyEvent::new_with_kind(code, KeyModifiers::NONE, KeyEventKind::Repeat)
    }

    #[test]
    fn page_down_repeat_is_dispatched() {
        assert!(dispatchable(repeated(KeyCode::PageDown)));
    }

    #[test]
    fn character_repeat_remains_filtered() {
        assert!(!dispatchable(repeated(KeyCode::Char('x'))));
    }
}
