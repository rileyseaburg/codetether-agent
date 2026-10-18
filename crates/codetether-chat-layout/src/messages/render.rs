//! Owned-buffer and borrowed-buffer message-panel rendering.
use super::{Host, clamp};
use codetether_tui_style::border_style::pane_border;
use ratatui::{Frame, layout::Rect, text::Line, widgets::Paragraph};
/// Render an owned line buffer with paragraph scroll.
/// # Arguments
/// Receives the frame, presentation host, panel rectangle and owned lines.
/// # Returns
/// Draws the panel into the current frame.
/// # Examples
/// ```no_run
/// fn draw<H: codetether_chat_layout::messages::Host>(f: &mut ratatui::Frame, host: &mut H) {
///     let area = f.area();
///     codetether_chat_layout::messages::render_owned(f, host, area, vec![]);
/// }
/// ```
pub fn render_owned(f: &mut Frame, host: &mut impl Host, area: Rect, lines: Vec<Line<'static>>) {
    let accent = host.accent();
    let block = pane_border(true)
        .border_style(ratatui::style::Style::default().fg(accent))
        .title(host.title());
    let scroll = clamp(host.scroll(), area, &lines);
    let chat = Paragraph::new(lines).block(block).scroll((scroll, 0));
    f.render_widget(chat, area);
}
/// Render the visible slice and record hit targets before drawing its scrollbar.
/// # Arguments
/// Receives the frame, presentation host, panel rectangle and borrowed lines.
/// # Returns
/// Draws the panel and updates hit geometry for the visible rows.
/// # Examples
/// ```no_run
/// fn draw<H: codetether_chat_layout::messages::Host>(f: &mut ratatui::Frame, host: &mut H) {
///     let area = f.area();
///     codetether_chat_layout::messages::render_ref(f, host, area, &[]);
/// }
/// ```
pub fn render_ref(f: &mut Frame, host: &mut impl Host, area: Rect, lines: &[Line<'static>]) {
    let accent = host.accent();
    let block = pane_border(true)
        .border_style(ratatui::style::Style::default().fg(accent))
        .title(host.title());
    let scroll = clamp(host.scroll(), area, lines);
    let visible = crate::visible_lines(lines, scroll, area.height);
    host.hit().record_visible(area, scroll, visible);
    let chat = Paragraph::new(visible.to_vec()).block(block);
    f.render_widget(chat, area);
    crate::scrollbar_render::render_scrollbar(f, area, lines.len(), scroll as usize);
}
