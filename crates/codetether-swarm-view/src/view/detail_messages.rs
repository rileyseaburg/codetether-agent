//! Swarm detail messages presentation.
use crate::SubTaskInfo;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
pub(super) fn append<'a>(task: &'a SubTaskInfo, content_lines: &mut Vec<Line<'a>>) {
    // Messages section
    if !task.messages.is_empty() {
        content_lines.push(Line::from(Span::styled(
            "─── Conversation ───",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )));
        content_lines.push(Line::from(""));

        for (i, _msg) in task.messages.iter().enumerate() {
            super::message_entry::append(task, i, content_lines);
        }
    }
}
