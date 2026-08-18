//! Response inspection helpers for the tool router.

use crate::provider::{CompletionResponse, ContentPart};

/// Whether the response already carries structured tool calls.
pub(super) fn has_tool_calls(response: &CompletionResponse) -> bool {
    response
        .message
        .content
        .iter()
        .any(|p| matches!(p, ContentPart::ToolCall { .. }))
}

/// Concatenate the assistant's visible text parts.
pub(super) fn assistant_text(response: &CompletionResponse) -> String {
    response
        .message
        .content
        .iter()
        .filter_map(|p| match p {
            ContentPart::Text { text } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n")
}
