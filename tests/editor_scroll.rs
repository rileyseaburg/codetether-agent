//! Tests for cursor-follow scroll offset logic.

use codetether_agent::tui::ui::editor::helix_backend::HelixBackend;
use codetether_agent::tui::ui::editor::scroll::follow_cursor;
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
