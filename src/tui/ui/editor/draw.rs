//! Draws the editor view to a ratatui [`Frame`].
//!
//! Thin wrapper over [`editor_lines`](super::render::editor_lines): it picks the
//! visible window from `scroll`, renders a titled bordered paragraph, and places
//! the terminal cursor. Pure layout/draw logic; no input or file concerns.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders, Paragraph};

use super::file_buffer::FileBuffer;
use super::render::editor_lines;
use super::scroll::follow_cursor;

/// Renders `buf` into `area`, scrolled so line `scroll` is at the top.
pub fn draw(f: &mut Frame, area: Rect, buf: &FileBuffer, scroll: usize) {
    let height = area.height.saturating_sub(2) as usize;
    let lines = editor_lines(buf.backend(), scroll, height);
    let dirty = if buf.is_dirty() { " *" } else { "" };
    let title = format!(" {}{} ", buf.path().display(), dirty);
    let block = Block::default().borders(Borders::ALL).title(title);
    f.render_widget(Paragraph::new(lines).block(block), area);
}

/// Draws the editor from app state if a buffer is active; no-op otherwise.
///
/// Updates `editor_scroll` so the cursor stays visible before drawing.
pub fn draw_active(f: &mut Frame, app: &mut crate::tui::app::state::App) {
    let height = f.area().height.saturating_sub(2) as usize;
    if let Some(buf) = app.state.editor.as_ref() {
        app.state.editor_scroll = follow_cursor(buf.backend(), app.state.editor_scroll, height);
    }
    if let Some(buf) = app.state.editor.as_ref() {
        draw(f, f.area(), buf, app.state.editor_scroll);
    }
}
