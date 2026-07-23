//! Provider-message formatting for managed-agent detail panes.

use ratatui::style::Stylize;
use ratatui::text::Line;

use crate::provider::{Message, Role};

#[path = "subagent_message_label.rs"]
mod label;
pub(super) use label::Source;

pub(super) fn append(rows: &mut Vec<Line<'static>>, messages: &[Message], source: Source) {
    if messages.is_empty() {
        rows.push(Line::from("No transcript events yet.".dim()));
        return;
    }
    for message in messages {
        if message.role == Role::System {
            rows.push(Line::from(
                format!(
                    "SYSTEM · runtime prompt hidden ({} chars)",
                    message_len(message)
                )
                .dim(),
            ));
            rows.push(Line::from(""));
            continue;
        }
        rows.push(Line::from(label::role(message.role, source)));
        let text = crate::tui::app::message_text::extract_message_text(&message.content);
        rows.extend(text.lines().map(|line| Line::from(format!("  {line}"))));
        rows.push(Line::from(""));
    }
}

fn message_len(message: &Message) -> usize {
    crate::tui::app::message_text::extract_message_text(&message.content)
        .chars()
        .count()
}

#[cfg(test)]
#[path = "subagent_message_lines_tests.rs"]
mod tests;
