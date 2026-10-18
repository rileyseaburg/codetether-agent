//! Tests for cursor-follow scroll offset logic.

use codetether_agent::tui::ui::editor::helix_backend::HelixBackend;
use codetether_agent::tui::ui::editor::scroll::{follow_col, follow_cursor};
use codetether_agent::tui::ui::editor::{EditorEdit, Move};

fn many_lines(n: usize) -> HelixBackend {
    let text = (0..n).map(|i| i.to_string()).collect::<Vec<_>>().join("\n");
    HelixBackend::from_str(&text)
}

#[test]
fn no_change_when_cursor_visible() {
    let b = many_lines(100); // cursor at line 0
    assert_eq!(follow_cursor(&b, 0, 10), 0);
}

#[test]
fn scrolls_down_to_keep_cursor_last_row() {
    let mut b = many_lines(100);
    for _ in 0..20 {
        b.move_cursor(Move::Down);
    }
    // Cursor at line 20, height 10 -> top should be 20 + 1 - 10 = 11.
    assert_eq!(follow_cursor(&b, 0, 10), 11);
}

#[test]
fn scrolls_up_when_cursor_above_window() {
    let mut b = many_lines(100);
    for _ in 0..5 {
        b.move_cursor(Move::Down);
    }
    // Cursor at line 5, but window starts at 40 -> jump up to 5.
    assert_eq!(follow_cursor(&b, 40, 10), 5);
}

#[test]
fn zero_height_is_noop() {
    let b = many_lines(10);
    assert_eq!(follow_cursor(&b, 7, 0), 7);
}

#[test]
fn follow_col_scrolls_right_then_holds() {
    let mut b = HelixBackend::from_str(&"x".repeat(200));
    for _ in 0..30 {
        b.move_cursor(Move::Right);
    }
    // cursor col 30, width 10 -> hscroll = 30 + 1 - 10 = 21
    assert_eq!(follow_col(&b, 0, 10), 21);
    // cursor still visible at hscroll 25 -> unchanged
    assert_eq!(follow_col(&b, 25, 10), 25);
}
