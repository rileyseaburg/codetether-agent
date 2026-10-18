//! Swarm detail tools presentation.
use crate::SubTaskInfo;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
pub(super) fn append<'a>(task: &'a SubTaskInfo, content_lines: &mut Vec<Line<'a>>) {
    // Tool call history section
    if !task.tool_call_history.is_empty() {
        content_lines.push(Line::from(Span::styled(
            "─── Tool Call History ───",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )));
        content_lines.push(Line::from(""));

        for (i, _tc) in task.tool_call_history.iter().enumerate() {
            super::tool_entry::append(task, i, content_lines);
        }
        content_lines.push(Line::from(""));
    } else if task.steps > 0 {
        // At least show that tool calls happened even without detail
        content_lines.push(Line::from(Span::styled(
            format!("─── {} tool calls (no detail captured) ───", task.steps),
            Style::default().fg(Color::DarkGray),
        )));
        content_lines.push(Line::from(""));
    }
}
