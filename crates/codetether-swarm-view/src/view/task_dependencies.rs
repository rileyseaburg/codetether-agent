//! Swarm task dependencies presentation.
use crate::SubTaskInfo;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
pub(super) fn row(task: &SubTaskInfo) -> Line<'_> {
    Line::from(vec![
        Span::styled("Deps: ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            if task.dependencies.is_empty() {
                "none".to_string()
            } else {
                task.dependencies.join(", ")
            },
            Style::default().fg(Color::DarkGray),
        ),
    ])
}
