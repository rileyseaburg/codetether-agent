//! Placeholder for an empty tool-activity panel.
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

pub(super) fn empty_body_lines() -> Vec<Line<'static>> {
    vec![Line::from(vec![
        Span::styled("│ ", Style::default().fg(Color::DarkGray).dim()),
        Span::styled(
            "No tool activity captured",
            Style::default().fg(Color::DarkGray).dim(),
        ),
    ])]
}
