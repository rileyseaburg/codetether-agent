//! Token-count estimation heuristics for swarm message budgeting.
//!
//! These are rough estimates (~3.5 chars/token) used to decide when to
//! truncate sub-agent message histories before a provider call.

use crate::provider::{ContentPart, Message};

/// Estimate token count from text (rough heuristic: ~3.5 chars per token).
pub(super) fn estimate_tokens(text: &str) -> usize {
    (text.len() as f64 / 3.5).ceil() as usize
}

/// Estimate total tokens in a single message, including role overhead.
pub(super) fn estimate_message_tokens(message: &Message) -> usize {
    let mut tokens = 4; // Role overhead

    for part in &message.content {
        tokens += match part {
            ContentPart::Text { text } => estimate_tokens(text),
            ContentPart::ToolCall {
                id,
                name,
                arguments,
                ..
            } => estimate_tokens(id) + estimate_tokens(name) + estimate_tokens(arguments) + 10,
            ContentPart::ToolResult {
                tool_call_id,
                content,
            } => estimate_tokens(tool_call_id) + estimate_tokens(content) + 6,
            ContentPart::Image { .. } | ContentPart::File { .. } => 2000,
            ContentPart::Thinking { text, .. } => estimate_tokens(text),
        };
    }

    tokens
}

/// Estimate total tokens across all messages.
pub(super) fn estimate_total_tokens(messages: &[Message]) -> usize {
    messages.iter().map(estimate_message_tokens).sum()
}
