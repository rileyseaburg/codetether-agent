//! Tests for editor editing/navigation key mappings ([`super::input::map_key`]).

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::edit::Move;
use super::input::{EditorInput, map_key};

fn plain(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}

#[test]
fn tab_indents_and_delete_removes_forward() {
    assert_eq!(map_key(plain(KeyCode::Tab)), Some(EditorInput::Indent));
    assert_eq!(
        map_key(plain(KeyCode::Delete)),
        Some(EditorInput::DeleteForward)
    );
}

#[test]
fn home_and_end_map_to_line_moves() {
    assert_eq!(
        map_key(plain(KeyCode::Home)),
        Some(EditorInput::Move(Move::LineStart))
    );
    assert_eq!(
        map_key(plain(KeyCode::End)),
        Some(EditorInput::Move(Move::LineEnd))
    );
}
