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

/// Inverse of [`cursor_xy`]: maps a screen cell to a logical `(line, col)`.
///
/// Returns `None` when the click lands on the border or gutter, outside any
/// text cell.
pub fn cell_to_cursor<B: EditorBackend>(
    backend: &B,
    area: Rect,
    scroll: usize,
    hscroll: usize,
    col: u16,
    row: u16,
) -> Option<(usize, usize)> {
    let gutter = gutter_width(backend.line_count()) + 1;
    let text_x0 = area.x + 1 + gutter as u16;
    let text_y0 = area.y + 1;
    if col < text_x0 || row < text_y0 || row + 1 >= area.y + area.height {
        return None;
    }
    let line = (row - text_y0) as usize + scroll;
    let column = (col - text_x0) as usize + hscroll;
    Some((line, column))
}
