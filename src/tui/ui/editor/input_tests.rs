//! Tests for editor key-to-action mapping ([`super::input::map_key`]).

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::edit::Move;
use super::input::{EditorInput, map_key};

fn ctrl(c: char) -> KeyEvent {
    KeyEvent::new(KeyCode::Char(c), KeyModifiers::CONTROL)
}

fn plain(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}

#[test]
fn ctrl_p_and_o_open_the_file_finder() {
    assert_eq!(map_key(ctrl('p')), Some(EditorInput::OpenFinder));
    assert_eq!(map_key(ctrl('o')), Some(EditorInput::OpenFinder));
}

#[test]
fn ctrl_s_still_saves() {
    assert_eq!(map_key(ctrl('s')), Some(EditorInput::Save));
}

#[test]
fn unmapped_ctrl_key_is_ignored() {
    assert_eq!(map_key(ctrl('z')), None);
}

#[test]
fn plain_keys_insert_and_navigate() {
    assert_eq!(
        map_key(plain(KeyCode::Char('a'))),
        Some(EditorInput::Insert('a'))
    );
    assert_eq!(map_key(plain(KeyCode::Esc)), Some(EditorInput::Quit));
    assert_eq!(
        map_key(plain(KeyCode::Left)),
        Some(EditorInput::Move(Move::Left))
    );
}
