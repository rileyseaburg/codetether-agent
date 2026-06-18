//! Vertical scrollbar column for the message pane.
//!
//! Produces a column of spans: a `█` thumb over a `│` track, sized to the
//! viewport height with the thumb positioned proportionally to the scroll
//! offset. Pure function — no application state.

use ratatui::{
    style::{Color, Style},
    text::Span,
};

/// Build a vertical scrollbar column, or empty when no scrolling is needed.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::ui::chat_view::scroll_indicator::scrollbar_column;
/// let col = scrollbar_column(100, 10, 0);
/// assert_eq!(col.len(), 10);
/// assert!(scrollbar_column(5, 10, 0).is_empty());
/// ```
pub fn scrollbar_column(
    total_lines: usize,
    viewport_height: usize,
    scroll_offset: usize,
) -> Vec<Span<'static>> {
    if total_lines <= viewport_height || viewport_height == 0 {
        return Vec::new();
    }
    let max_offset = total_lines - viewport_height;
    let offset = scroll_offset.min(max_offset);
    let thumb = (offset * (viewport_height.saturating_sub(1))) / max_offset.max(1);
    (0..viewport_height)
        .map(|row| {
            if row == thumb {
                Span::styled("█", Style::default().fg(Color::Cyan))
            } else {
                Span::styled("│", Style::default().fg(Color::DarkGray))
            }
        })
        .collect()
}
