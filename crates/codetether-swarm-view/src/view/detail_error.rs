//! Swarm detail error presentation.
use crate::SubTaskInfo;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
pub(super) fn append<'a>(task: &'a SubTaskInfo, content_lines: &mut Vec<Line<'a>>) {
    // Error section
    if let Some(ref err) = task.error {
        content_lines.push(Line::from(Span::styled(
            "─── Error ───",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )));
        content_lines.push(Line::from(""));
        for line in err.lines() {
            content_lines.push(Line::from(Span::styled(
                line,
                Style::default().fg(Color::Red),
            )));
        }
        content_lines.push(Line::from(""));
    }
}
