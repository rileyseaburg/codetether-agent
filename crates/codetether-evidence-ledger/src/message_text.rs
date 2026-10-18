//! Collect text, tool output and thinking parts for evidence extraction.
use codetether_provider_core::{ContentPart, Message};

/// Join text, tool-result and thinking content in conversation order.
/// # Arguments
/// `messages` supplies provider messages.
/// # Returns
/// Text separated by newlines, excluding other content parts.
/// # Examples
/// ```rust
/// assert!(codetether_evidence_ledger::message_text::joined(&[]).is_empty());
/// ```
pub fn joined(messages: &[Message]) -> String {
    messages
        .iter()
        .flat_map(|msg| msg.content.iter())
        .filter_map(|part| match part {
            ContentPart::Text { text } => Some(text.as_str()),
            ContentPart::ToolResult { content, .. } => Some(content.as_str()),
            ContentPart::Thinking { text, .. } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n")
}
