//! Selected story status, priority and step count.
use crate::RalphStoryInfo;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
pub(super) fn row(
    story: &RalphStoryInfo,
    status_text: &'static str,
    status_color: Color,
) -> Line<'static> {
    Line::from(vec![
        Span::styled("Status: ", Style::default().fg(Color::DarkGray)),
        Span::styled(status_text, Style::default().fg(status_color)),
        Span::raw("  "),
        Span::styled("Priority: ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{}", story.priority),
            Style::default().fg(Color::White),
        ),
        Span::raw("  "),
        Span::styled("Steps: ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{}", story.steps),
            Style::default().fg(Color::White),
        ),
    ])
}
