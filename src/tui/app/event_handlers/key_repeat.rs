//! Key-event repeat policy for actions that should respond while held.

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

pub(super) fn dispatchable(key: KeyEvent) -> bool {
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
