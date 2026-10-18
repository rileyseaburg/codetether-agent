//! Help separator content and presentation.
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
pub(crate) fn separator() -> Line<'static> {
    Line::from(Span::styled(
        "  ─────────────────────────────────────────────",
        Style::default().fg(Color::DarkGray),
    ))
}
