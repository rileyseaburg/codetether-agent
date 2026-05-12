//! Styling helpers for the TUI git view.

use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

/// Render a cyan section heading.
pub fn heading(text: &str) -> Line<'static> {
    Line::from(Span::styled(
        text.to_string(),
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ))
}
