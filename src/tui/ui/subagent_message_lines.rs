//! Provider-message formatting for managed-agent detail panes.

use ratatui::style::Stylize;
use ratatui::text::Line;

use crate::provider::{Message, Role};

pub(super) fn append(rows: &mut Vec<Line<'static>>, messages: &[Message]) {
    if messages.is_empty() {
        rows.push(Line::from("No transcript events yet.".dim()));
        return;
    }
    for message in messages {
        rows.push(Line::from(role_label(message.role)));
        let text = crate::tui::app::message_text::extract_message_text(&message.content);
        rows.extend(text.lines().map(|line| Line::from(format!("  {line}"))));
        rows.push(Line::from(""));
    }
}

fn role_label(role: Role) -> ratatui::text::Span<'static> {
    match role {
        Role::User => "USER".to_string().bold(),
        Role::Assistant => "ASSISTANT".cyan().bold(),
        Role::System => "SYSTEM".dim().bold(),
        Role::Tool => "TOOL RESULT".magenta().bold(),
    }
}
