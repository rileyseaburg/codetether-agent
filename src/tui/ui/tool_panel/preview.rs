//! Multi-line text preview helpers used by the tool activity panel.

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

use super::preview_excerpt::preview_excerpt;

pub(super) fn push_preview_lines(
    body_lines: &mut Vec<Line<'static>>,
    text: &str,
    preview_width: usize,
    style: Style,
    empty_label: &str,
) {
    let preview = preview_excerpt(text, preview_width);
    if preview.lines.is_empty() {
        body_lines.push(prefix_line(empty_label.to_string(), style));
        return;
    }

    for line in preview.lines {
        body_lines.push(prefix_line(line, style));
    }

    if preview.truncated {
        body_lines.push(prefix_line(
            "…".to_string(),
            Style::default().fg(Color::DarkGray).dim(),
        ));
    }
}

fn prefix_line(text: String, style: Style) -> Line<'static> {
    Line::from(vec![
        Span::styled("│   ", Style::default().fg(Color::DarkGray).dim()),
        Span::styled(text, style),
    ])
}
