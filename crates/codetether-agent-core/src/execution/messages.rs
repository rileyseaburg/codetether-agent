//! Extract model tool calls and final response text without session state.

use codetether_provider_core::{ContentPart, Message};

/// Extracts the canonical tool-call content parts from a provider message.
///
/// Returns owned [`ContentPart::ToolCall`] values (preserving
/// `thought_signature`) rather than flattening into positional tuples.
///
/// # Examples
///
/// ```ignore
/// let calls = collect_tool_calls(&message);
/// ```
/// # Arguments
/// * `message` — Provider message to inspect.
/// # Returns
/// Owned tool-call parts in their original order, including signatures.
/// # Examples
/// ```
/// use codetether_provider_core::{Message, Role};
/// let message = Message { role: Role::Assistant, content: vec![] };
/// assert!(codetether_agent_core::execution::messages::collect_tool_calls(&message).is_empty());
/// ```
pub fn collect_tool_calls(message: &Message) -> Vec<ContentPart> {
    message
        .content
        .iter()
        .filter(|p| p.as_tool_call().is_some())
        .cloned()
        .collect()
}

/// Extracts the concatenated text content from a provider message.
///
/// # Examples
///
/// ```ignore
/// let text = response_text(&message);
/// ```
/// # Arguments
/// * `message` — Provider message whose text should be extracted.
/// # Returns
/// Text parts joined with newlines.
/// # Examples
/// ```
/// use codetether_provider_core::{Message, Role};
/// let message = Message { role: Role::Assistant, content: vec![] };
/// assert_eq!(codetether_agent_core::execution::messages::response_text(&message), "");
/// ```
pub fn response_text(message: &Message) -> String {
    message
        .content
        .iter()
        .filter_map(extract_text)
        .collect::<Vec<_>>()
        .join("\n")
}

fn extract_text(part: &ContentPart) -> Option<String> {
    match part {
        ContentPart::Text { text } => Some(text.clone()),
        _ => None,
    }
}
