//! Gemini Web conversation-history rendering.

mod catalog;
mod dedupe;
mod part;
mod protocol;
mod retry;
mod window;

use crate::provider::{Message, Role, ToolDefinition};
use anyhow::Result;

/// Render messages using the same XML protocol expected from model responses.
pub(super) fn render(messages: &[Message], tools: &[ToolDefinition]) -> Result<String> {
    let catalog = catalog::render(tools)?;
    let overhead = protocol::overhead(catalog.len()) + retry::RESERVE_BYTES;
    let history_limit = window::MAX_BYTES.saturating_sub(overhead);
    let history = render_bounded(messages, history_limit);
    Ok(protocol::wrap(&catalog, &history))
}

pub(super) fn retry(original: &str, error: &str) -> String {
    retry::render(original, error)
}

pub(super) fn render_bounded(messages: &[Message], max_bytes: usize) -> String {
    let rendered = messages.iter().map(render_message).collect::<Vec<_>>();
    let (messages, rendered) = dedupe::system_messages(messages, rendered);
    window::fit(&messages, rendered, max_bytes).join("\n")
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
