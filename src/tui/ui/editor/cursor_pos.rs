//! Computes the terminal cursor position for the editor view.
//!
//! [`cursor_xy`] maps the editor's logical `(line, col)` cursor into absolute
//! screen coordinates inside `area`, accounting for the 1-cell border, the
//! line-number gutter, and the current scroll offset. Returns `None` when the
//! cursor is scrolled out of the visible window.

use ratatui::layout::Rect;

use super::backend::EditorBackend;
use super::render::gutter_width;

/// Absolute `(x, y)` screen cell for the cursor, or `None` if off-screen.
pub fn cursor_xy<B: EditorBackend>(
    backend: &B,
    area: Rect,
    scroll: usize,
    hscroll: usize,
) -> Option<(u16, u16)> {
    let (line, col) = backend.cursor();
    let height = area.height.saturating_sub(2) as usize;
    if line < scroll || line >= scroll + height || col < hscroll {
        return None;
    }
    // Gutter is `width` digits plus one trailing space; +1 for the left border.
    let gutter = gutter_width(backend.line_count()) + 1;
    let x = area.x + 1 + gutter as u16 + (col - hscroll) as u16;
    let y = area.y + 1 + (line - scroll) as u16;
    Some((x, y))
}
