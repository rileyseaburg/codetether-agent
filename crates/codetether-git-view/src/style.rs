//! Styling helpers for the TUI git view.

use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

/// Render a cyan section heading.
/// # Arguments
/// * `text` - Section heading text.
/// # Returns
/// A cyan bold heading line.
/// # Examples
/// ```no_run
/// let _line = codetether_git_view::heading("Recent commits");
/// ```
pub fn heading(text: &str) -> Line<'_> {
    Line::from(Span::styled(
        text,
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ))
}
