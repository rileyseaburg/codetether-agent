//! Gemini Web conversation-history rendering.

mod part;
mod window;

use crate::provider::{Message, Role};

/// Render messages using the same XML protocol expected from model responses.
pub(super) fn render(messages: &[Message]) -> String {
    render_bounded(messages, window::MAX_BYTES)
}

pub(super) fn render_bounded(messages: &[Message], max_bytes: usize) -> String {
    let rendered = messages.iter().map(render_message).collect::<Vec<_>>();
    window::fit(messages, rendered, max_bytes).join("\n")
}

fn render_message(message: &Message) -> String {
    let role = match message.role {
        Role::System | Role::Developer => "System",
        Role::User => "User",
        Role::Assistant => "Assistant",
        Role::Tool => "Tool",
    };
    let body = message
        .content
        .iter()
        .filter_map(part::render)
        .collect::<Vec<_>>()
        .join("\n");
    format!("{role}: {body}")
}
