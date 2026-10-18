//! Unicode cursor and edit regressions for the owned composer.
use crate::ComposerState;

#[test]
fn utf8_cursor_moves_by_character_not_byte() {
    let mut state = ComposerState::default();
    state.input = "aé🙂z".to_string();
    state.move_cursor_end();
    assert_eq!(state.cursor, 4);
    state.move_cursor_left();
    assert_eq!(state.cursor, 3);
    state.move_cursor_left();
    assert_eq!(state.cursor, 2);
    state.move_cursor_right();
    assert_eq!(state.cursor, 3);
}

#[test]
fn utf8_backspace_removes_full_character() {
    let mut state = ComposerState::default();
    state.input = "aé🙂z".to_string();
    state.move_cursor_end();
    state.move_cursor_left();
    state.delete_backspace();
    assert_eq!(state.input, "aéz");
    assert_eq!(state.cursor, 2);
}

#[test]
fn utf8_delete_forward_removes_full_character() {
    let mut state = ComposerState::default();
    state.input = "aé🙂z".to_string();
    state.cursor = 1;
    state.delete_forward();
    assert_eq!(state.input, "a🙂z");
    assert_eq!(state.cursor, 1);
}

#[test]
fn utf8_insert_respects_character_cursor() {
    let mut state = ComposerState::default();
    state.input = "a🙂z".to_string();
    state.cursor = 2;
    state.insert_char('é');
    assert_eq!(state.input, "a🙂éz");
    assert_eq!(state.cursor, 3);
}
