//! Draw a file buffer into the target terminal rectangle.
use super::{cursor_pos::cursor_xy, file_buffer::FileBuffer, render::editor_lines};
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
};

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
