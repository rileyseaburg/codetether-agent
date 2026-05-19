//! Render a `Thinking` row.

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

use super::preview::push_preview_lines;

pub(super) fn render_thinking(
    body_lines: &mut Vec<Line<'static>>,
    timestamp: &str,
    thoughts: &str,
    preview_width: usize,
) {
    let style = Style::default().fg(Color::DarkGray).dim().italic();
    body_lines.push(Line::from(vec![
        Span::styled("│ ", Style::default().fg(Color::DarkGray).dim()),
        Span::styled(format!("[{timestamp}] 💭 thinking"), style),
    ]));
    push_preview_lines(
        body_lines,
        thoughts,
        preview_width,
        style,
        "(no reasoning text)",
    );
}
