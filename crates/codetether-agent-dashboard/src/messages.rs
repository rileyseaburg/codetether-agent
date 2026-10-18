//! Provider-message formatting for managed-agent detail panes.

use ratatui::style::Stylize;
use ratatui::text::Line;

use codetether_provider_core::{Message, Role};

use crate::message_label as label;
pub use label::Source;

/// Append provider messages, hiding runtime system prompts.
/// # Arguments
/// * `rows` - Destination detail rows.
/// * `messages` - Provider messages to format.
/// * `source` - Origin controlling role labels.
/// # Returns
/// Appends transcript rows or the empty-transcript hint.
/// # Examples
/// ```
/// let mut rows = Vec::new();
/// codetether_agent_dashboard::messages::append(&mut rows, &[], codetether_agent_dashboard::messages::Source::ManagedAgent);
/// assert!(!rows.is_empty());
/// ```
pub fn append(rows: &mut Vec<Line<'static>>, messages: &[Message], source: Source) {
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
        let text = codetether_chat_transcript::extract_message_text(&message.content);
        rows.extend(text.lines().map(|line| Line::from(format!("  {line}"))));
        rows.push(Line::from(""));
    }
}

fn message_len(message: &Message) -> usize {
    codetether_chat_transcript::extract_message_text(&message.content)
        .chars()
        .count()
}

#[cfg(test)]
#[path = "messages_tests.rs"]
mod tests;
