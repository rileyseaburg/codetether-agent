//! Swarm task identity presentation.
use crate::SubTaskInfo;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
pub(super) fn row(task: &SubTaskInfo) -> Line<'_> {
    Line::from(vec![
        Span::styled("Task: ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            &task.name,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
    ])
}
