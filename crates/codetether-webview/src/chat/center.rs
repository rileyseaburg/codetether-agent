//! Transcript rendering and bounded vertical scrolling.
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

/// Render transcript lines at the requested scroll offset.
/// # Arguments
/// * `f` - Destination frame.
/// * `chat_scroll` - Requested row offset, clamped to available lines.
/// * `area` - Transcript rectangle.
/// * `lines` - Already-formatted transcript rows.
/// # Returns
/// Draws the visible transcript without a border.
/// # Examples
/// ```no_run
/// fn draw(f: &mut ratatui::Frame, area: ratatui::layout::Rect) {
///     codetether_webview::chat::render_center(f, 0, area, &[]);
/// }
/// ```
pub fn render_center(f: &mut Frame, chat_scroll: usize, area: Rect, lines: &[Line<'static>]) {
    let visible = area.height.saturating_sub(2) as usize;
    let scroll = chat_scroll.min(lines.len().saturating_sub(visible));
    let block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default().fg(Color::default()));
    let para = Paragraph::new(lines.to_vec())
        .block(block)
        .scroll((scroll.min(u16::MAX as usize) as u16, 0));
    f.render_widget(para, area);
}
