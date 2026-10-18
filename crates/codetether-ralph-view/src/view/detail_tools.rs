//! Ralph detail tools presentation.
use crate::RalphStoryInfo;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
pub(super) fn append<'a>(story: &'a RalphStoryInfo, content_lines: &mut Vec<Line<'a>>) {
    // Tool call history
    if !story.tool_call_history.is_empty() {
        content_lines.push(Line::from(Span::styled(
            "─── Tool Call History ───",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )));
        content_lines.push(Line::from(""));

        for (i, _tc) in story.tool_call_history.iter().enumerate() {
            super::tool_entry::append(story, i, content_lines);
        }
        content_lines.push(Line::from(""));
    } else if story.steps > 0 {
        content_lines.push(Line::from(Span::styled(
            format!("─── {} tool calls (no detail captured) ───", story.steps),
            Style::default().fg(Color::DarkGray),
        )));
        content_lines.push(Line::from(""));
    }
}
