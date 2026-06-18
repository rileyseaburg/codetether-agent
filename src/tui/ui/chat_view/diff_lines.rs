//! Diff-aware line coloring for tool output.
//!
//! Colors a single line of unified-diff text: additions green, deletions
//! red, hunk headers cyan, and file headers bold. Pure function — safe to
//! call per line while rendering cached tool-result blocks.

use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

/// Colorize a single unified-diff line.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::ui::chat_view::diff_lines::colorize_diff_line;
/// use ratatui::style::Color;
/// let line = colorize_diff_line("+added");
/// assert_eq!(line.spans[0].style.fg, Some(Color::Green));
/// ```
pub fn colorize_diff_line(line: &str) -> Line<'static> {
    let style = if line.starts_with("+++") || line.starts_with("---") {
        Style::default().add_modifier(Modifier::BOLD)
    } else if line.starts_with("@@") {
        Style::default().fg(Color::Cyan)
    } else if line.starts_with('+') {
        Style::default().fg(Color::Green)
    } else if line.starts_with('-') {
        Style::default().fg(Color::Red)
    } else {
        Style::default()
    };
    Line::from(Span::styled(line.to_string(), style))
}
