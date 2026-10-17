use super::constants::CHARS_PER_TOKEN;
use codetether_provider_core::{ContentPart, Message};

/// Estimate tokens from UTF-8 byte length using the existing 3.5 ratio.
///
/// # Arguments
/// * `text` - Text to estimate.
///
/// # Returns
/// The rounded-up estimate.
///
/// # Examples
/// ```
/// assert_eq!(codetether_swarm_core::context::estimate_tokens("hello"), 2);
/// ```
pub fn estimate_tokens(text: &str) -> usize {
    (text.len() as f64 / CHARS_PER_TOKEN).ceil() as usize
}

/// Estimate a message including role, content and tool overhead.
///
/// # Arguments
/// * `message` - Structured provider message.
///
/// # Returns
/// Total estimated tokens for the message.
///
/// # Examples
/// See [estimate_total_tokens] for estimating a history.
pub fn estimate_message_tokens(message: &Message) -> usize {
    let mut tokens = 4;
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

/// Sum estimated tokens across a message history.
///
/// # Arguments
/// * `messages` - Messages to estimate.
///
/// # Returns
/// The sum of per-message estimates.
///
/// # Examples
/// ```
/// assert_eq!(codetether_swarm_core::context::estimate_total_tokens(&[]), 0);
/// ```
pub fn estimate_total_tokens(messages: &[Message]) -> usize {
    messages.iter().map(estimate_message_tokens).sum()
}
