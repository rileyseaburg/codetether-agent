//! Integration tests for editor key-mapping and action application.

use codetether_agent::tui::ui::editor::apply::apply;
use codetether_agent::tui::ui::editor::input::EditorInput;
use codetether_agent::tui::ui::editor::{map_key, EditorBackend, FileBuffer, Move};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

fn key(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}

#[test]
fn maps_common_keys() {
    assert_eq!(map_key(key(KeyCode::Char('z'))), Some(EditorInput::Insert('z')));
    assert_eq!(map_key(key(KeyCode::Enter)), Some(EditorInput::Newline));
    assert_eq!(map_key(key(KeyCode::Backspace)), Some(EditorInput::Backspace));
    assert_eq!(map_key(key(KeyCode::Left)), Some(EditorInput::Move(Move::Left)));
    assert_eq!(map_key(key(KeyCode::Esc)), Some(EditorInput::Quit));
    assert_eq!(
        map_key(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::CONTROL)),
        Some(EditorInput::Save)
    );
}

#[test]
fn type_then_save_via_actions() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("f.txt");
    let mut buf = FileBuffer::open(&path).unwrap();

    for c in ['h', 'i'] {
        assert!(apply(&mut buf, EditorInput::Insert(c)).unwrap());
    }
    assert!(apply(&mut buf, EditorInput::Save).unwrap());
    // Quit returns false (editor should close).
    assert!(!apply(&mut buf, EditorInput::Quit).unwrap());

    assert_eq!(buf.backend().line_count(), 1);
    assert_eq!(std::fs::read_to_string(&path).unwrap(), "hi");
}
