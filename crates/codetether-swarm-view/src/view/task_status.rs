//! Swarm task status presentation.
use crate::SubTaskInfo;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
pub(super) fn row<'a>(
    task: &'a SubTaskInfo,
    agent_label: &'a str,
    status_icon: &'static str,
    status_color: Color,
) -> Line<'a> {
    Line::from(vec![
        Span::styled("Agent: ", Style::default().fg(Color::DarkGray)),
        Span::styled(agent_label, Style::default().fg(Color::Cyan)),
        Span::raw("  "),
        Span::styled("Status: ", Style::default().fg(Color::DarkGray)),
        Span::styled(status_icon, Style::default().fg(status_color)),
        Span::raw("  "),
        Span::styled("Stage: ", Style::default().fg(Color::DarkGray)),
        Span::styled(format!("{}", task.stage), Style::default().fg(Color::White)),
        Span::raw("  "),
        Span::styled("Steps: ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{}/{}", task.steps, task.max_steps),
            Style::default().fg(Color::White),
        ),
    ])
}
