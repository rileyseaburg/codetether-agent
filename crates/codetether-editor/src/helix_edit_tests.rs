//! Behavior tests for new [`HelixBackend`] edit/navigation operations.

use super::backend::EditorBackend;
use super::edit::{EditorEdit, Move};
use super::helix_backend::HelixBackend;

#[test]
fn delete_forward_removes_char_at_cursor() {
    let mut b = HelixBackend::from_str("abc\n");
    // cursor at (0,0)
    b.delete_forward();
    assert_eq!(b.to_text(), "bc\n");
    assert_eq!(b.cursor(), (0, 0));
}

#[test]
fn delete_forward_at_eof_is_noop() {
    let mut b = HelixBackend::from_str("a");
    b.move_cursor(Move::Right);
    b.delete_forward();
    assert_eq!(b.to_text(), "a");
}

#[test]
fn line_end_stops_before_trailing_newline() {
    let mut b = HelixBackend::from_str("hello\n");
    b.move_cursor(Move::LineEnd);
    assert_eq!(b.cursor(), (0, 5));
    b.insert_char('!');
    assert_eq!(b.to_text(), "hello!\n");
}

#[test]
fn line_start_returns_to_column_zero() {
    let mut b = HelixBackend::from_str("hello\n");
    b.move_cursor(Move::LineEnd);
    b.move_cursor(Move::LineStart);
    assert_eq!(b.cursor(), (0, 0));
}
