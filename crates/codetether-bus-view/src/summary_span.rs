//! Colored status and separator spans for protocol summaries.
use ratatui::{
    style::{Style, Stylize},
    text::Span,
};

pub(super) fn colored(text: &'static str, color: ratatui::style::Color) -> Span<'static> {
    Span::styled(text, Style::default().fg(color).bold())
}

pub(super) fn sep() -> Span<'static> {
    "  •  ".dim()
}
