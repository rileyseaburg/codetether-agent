//! Tests for editor terminal cursor positioning.

use codetether_agent::tui::ui::editor::cursor_pos::cursor_xy;
use codetether_agent::tui::ui::editor::helix_backend::HelixBackend;
use codetether_agent::tui::ui::editor::{EditorEdit, Move};
use ratatui::layout::Rect;

fn area() -> Rect {
    Rect::new(0, 0, 80, 24)
}

#[test]
fn cursor_at_origin_accounts_for_border_and_gutter() {
    // 3 lines -> gutter width 1, so prefix = border(1) + gutter(1) + space(1) = 3.
    let b = HelixBackend::from_str("a\nb\nc\n");
    assert_eq!(cursor_xy(&b, area(), 0), Some((3, 1)));
}

#[test]
fn cursor_tracks_column_and_row() {
    let mut b = HelixBackend::from_str("hello\nworld\n");
    b.move_cursor(Move::Down); // line 1
    b.move_cursor(Move::Right); // col 1
    b.move_cursor(Move::Right); // col 2
    // x = 1 border + 2 gutter+space + 2 col = 5 ; y = 1 border + 1 row = 2
    assert_eq!(cursor_xy(&b, area(), 0), Some((5, 2)));
}

#[test]
fn off_screen_cursor_returns_none() {
    let text = (0..100).map(|i| i.to_string()).collect::<Vec<_>>().join("\n");
    let b = HelixBackend::from_str(&text); // cursor at line 0
    // Window scrolled far past line 0 -> not visible.
    assert_eq!(cursor_xy(&b, area(), 50), None);
}
