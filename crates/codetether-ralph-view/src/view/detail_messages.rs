//! Ralph detail messages presentation.
use crate::RalphStoryInfo;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
pub(super) fn append<'a>(story: &'a RalphStoryInfo, content_lines: &mut Vec<Line<'a>>) {
    // Messages
    if !story.messages.is_empty() {
        content_lines.push(Line::from(Span::styled(
            "─── Conversation ───",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )));
        content_lines.push(Line::from(""));

        for (i, _msg) in story.messages.iter().enumerate() {
            super::message_entry::append(story, i, content_lines);
        }
    }
}
