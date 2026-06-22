//! Tests for editor screen-cell ↔ cursor coordinate mapping.

use ratatui::layout::Rect;

use super::cursor_pos::{cell_to_cursor, cursor_xy};
use super::helix_backend::HelixBackend;

fn area() -> Rect {
    Rect {
        x: 0,
        y: 0,
        width: 40,
        height: 10,
    }
}

#[test]
fn cell_maps_back_to_cursor_round_trip() {
    let backend = HelixBackend::from_str("hello\nworld\n");
    // Single-digit line count → gutter width 1, +1 trailing = text starts at x=2 (+1 border)=3.
    let (line, col) = cell_to_cursor(&backend, area(), 0, 0, 5, 2).expect("inside text");
    assert_eq!((line, col), (1, 2));
}

#[test]
fn cell_on_border_returns_none() {
    let backend = HelixBackend::from_str("abc\n");
    assert!(cell_to_cursor(&backend, area(), 0, 0, 0, 0).is_none());
}

#[test]
fn cursor_xy_and_cell_to_cursor_are_inverse() {
    let mut backend = HelixBackend::from_str("alpha\nbeta\ngamma\n");
    backend.set_cursor((2, 3));
    let (x, y) = cursor_xy(&backend, area(), 0, 0).expect("on screen");
    let (line, col) = cell_to_cursor(&backend, area(), 0, 0, x, y).expect("inside");
    assert_eq!((line, col), (2, 3));
}
