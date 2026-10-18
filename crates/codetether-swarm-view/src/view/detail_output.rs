//! Swarm detail output presentation.
use crate::SubTaskInfo;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
pub(super) fn append<'a>(task: &'a SubTaskInfo, content_lines: &mut Vec<Line<'a>>) {
    // Output section
    if let Some(ref output) = task.output {
        content_lines.push(Line::from(Span::styled(
            "─── Output ───",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )));
        content_lines.push(Line::from(""));
        for line in output.lines().take(20) {
            content_lines.push(Line::from(Span::styled(
                line,
                Style::default().fg(Color::White),
            )));
        }
        if output.lines().count() > 20 {
            content_lines.push(Line::from(Span::styled(
                "... (truncated)",
                Style::default().fg(Color::DarkGray),
            )));
        }
        content_lines.push(Line::from(""));
    }
}
