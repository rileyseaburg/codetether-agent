//! Render the body lines of a fenced code block.
//!
//! For `diff`/`patch` languages each line is colorized via
//! [`crate::tui::ui::chat_view::diff_lines::colorize_diff_line`]; otherwise
//! lines are passed through with the default dim code styling. Extracted
//! from `message_formatter` to keep that module within the line budget.

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

use crate::tui::ui::chat_view::diff_lines::colorize_diff_line;

/// Build the bordered body lines for a code block.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::message_formatter::message_formatter_code_render::render_code_body;
/// let out = render_code_body(&["+added".to_string()], "diff");
/// assert_eq!(out.len(), 1);
/// ```
pub fn render_code_body(lines: &[String], language: &str) -> Vec<Line<'static>> {
    let is_diff = matches!(language, "diff" | "patch");
    lines
        .iter()
        .map(|l| {
            let trimmed = l.trim_end();
            if is_diff && !trimmed.is_empty() {
                prefix_diff(trimmed)
            } else if trimmed.is_empty() {
                Line::from(Span::styled("│", Style::default().fg(Color::DarkGray)))
            } else {
                Line::from(Span::styled(
                    format!("│ {trimmed}"),
                    Style::default().fg(Color::DarkGray),
                ))
            }
        })
        .collect()
}

fn prefix_diff(text: &str) -> Line<'static> {
    let colored = colorize_diff_line(text);
    let mut spans = vec![Span::styled("│ ", Style::default().fg(Color::DarkGray))];
    spans.extend(colored.spans);
    Line::from(spans)
}
