//! Timeline gutter: vertical thread connecting conversation messages.
//!
//! Prepends a colored `│` rail to body lines and a `◉` anchor dot to
//! header lines, giving the chat a vertical-timeline reading rhythm
//! instead of flat stacked paragraphs.

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

/// Prefix a header line with the timeline anchor dot.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::ui::chat_view::timeline::anchor_line;
/// use ratatui::style::Color;
/// let line = anchor_line(ratatui::text::Line::from("hdr"), Color::Cyan);
/// assert!(line.spans.len() >= 2);
/// ```
pub fn anchor_line(header: Line<'static>, color: Color) -> Line<'static> {
    let mut spans = vec![Span::styled("◉ ", Style::default().fg(color))];
    spans.extend(header.spans);
    Line::from(spans)
}

/// Prefix a body line with the timeline rail.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::ui::chat_view::timeline::rail_line;
/// use ratatui::style::Color;
/// let line = rail_line(ratatui::text::Line::from("body"), Color::Cyan);
/// assert!(line.spans.len() >= 2);
/// ```
pub fn rail_line(body: Line<'static>, color: Color) -> Line<'static> {
    let mut spans = vec![Span::styled("│ ", Style::default().fg(color).dim())];
    spans.extend(body.spans);
    Line::from(spans)
}
