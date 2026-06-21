//! Draws the editor view to a ratatui [`Frame`].
//!
//! Thin wrapper over [`editor_lines`](super::render::editor_lines): it picks the
//! visible window from `scroll`, renders a titled bordered paragraph, and places
//! the terminal cursor. Pure layout/draw logic; no input or file concerns.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders, Paragraph};

use super::cursor_pos::cursor_xy;
use super::file_buffer::FileBuffer;
use super::render::{editor_lines, text_width};
use super::scroll::{follow_col, follow_cursor};

/// Renders `buf` into `area`, scrolled by `scroll` rows and `hscroll` columns.
pub fn draw(f: &mut Frame, area: Rect, buf: &FileBuffer, scroll: usize, hscroll: usize) {
    let height = area.height.saturating_sub(2) as usize;
    let lines = editor_lines(buf.backend(), scroll, height, hscroll);
    let dirty = if buf.is_dirty() { " *" } else { "" };
    let title = format!(" {}{} ", buf.path().display(), dirty);
    let block = Block::default().borders(Borders::ALL).title(title);
    f.render_widget(Paragraph::new(lines).block(block), area);
    if let Some((x, y)) = cursor_xy(buf.backend(), area, scroll, hscroll) {
        f.set_cursor_position((x, y));
    }
}

/// Draws the editor from app state if a buffer is active; no-op otherwise.
///
/// Updates `editor_scroll`/`editor_hscroll` so the cursor stays visible.
pub fn draw_active(f: &mut Frame, app: &mut crate::tui::app::state::App) {
    let area = f.area();
    let height = area.height.saturating_sub(2) as usize;
    if let Some(buf) = app.state.editor.as_ref() {
        let w = text_width(buf.backend(), area.width);
        app.state.editor_scroll = follow_cursor(buf.backend(), app.state.editor_scroll, height);
        app.state.editor_hscroll = follow_col(buf.backend(), app.state.editor_hscroll, w);
    }
    if let Some(buf) = app.state.editor.as_ref() {
        draw(f, area, buf, app.state.editor_scroll, app.state.editor_hscroll);
    }
}
