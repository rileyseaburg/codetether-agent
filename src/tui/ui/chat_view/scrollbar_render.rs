//! Overlay the vertical scrollbar column on the messages pane.
//!
//! Renders the thumb/track column produced by
//! [`super::scroll_indicator::scrollbar_column`] onto the right-hand
//! interior edge of the messages rect. No-op when the content fits, so
//! it costs nothing until the buffer overflows the viewport.

use ratatui::{Frame, layout::Rect, text::Line, widgets::Paragraph};

use super::scroll_indicator::scrollbar_column;

/// Draw the scrollbar inside `messages_rect` for the given line buffer.
///
/// # Examples
///
/// ```rust,no_run
/// use codetether_agent::tui::ui::chat_view::scrollbar_render::render_scrollbar;
/// # fn d(f: &mut ratatui::Frame) {
/// render_scrollbar(f, ratatui::layout::Rect::new(0, 0, 80, 24), 200, 0);
/// # }
/// ```
pub fn render_scrollbar(f: &mut Frame, messages_rect: Rect, total_lines: usize, scroll: usize) {
    let viewport = messages_rect.height.saturating_sub(2) as usize;
    let column = scrollbar_column(total_lines, viewport, scroll);
    if column.is_empty() || messages_rect.width < 2 {
        return;
    }
    let x = messages_rect.x + messages_rect.width - 1;
    let lines: Vec<Line<'static>> = column.into_iter().map(Line::from).collect();
    let area = Rect::new(x, messages_rect.y + 1, 1, viewport as u16);
    f.render_widget(Paragraph::new(lines), area);
}
