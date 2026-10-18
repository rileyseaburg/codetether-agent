//! Small styling/measurement helpers shared by the table renderer.

use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};
use unicode_width::UnicodeWidthStr;

/// Bold header cell style.
pub(super) fn header_style() -> Style {
    Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::BOLD)
}

/// Display width of a string, in terminal columns.
pub(super) fn w(s: &str) -> usize {
    UnicodeWidthStr::width(s)
}

/// Build a styled span from owned text.
pub(super) fn span(text: String, style: Style) -> Span<'static> {
    Span::styled(text, style)
}

/// Convenience: a single-span line.
pub(super) fn line(spans: Vec<Span<'static>>) -> Line<'static> {
    Line::from(spans)
}
