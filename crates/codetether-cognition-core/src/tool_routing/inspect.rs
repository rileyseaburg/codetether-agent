//! Response inspection helpers for the tool router.

use codetether_provider_core::{CompletionResponse, ContentPart};

/// Whether the response already carries structured tool calls.
///
/// # Arguments
/// * `response` - Provider completion response.
///
/// # Returns
/// The formatted or selected result described above.
///
/// # Examples
/// ```
/// # use codetether_provider_core::{ToolDefinition, CompletionResponse};
/// # fn example(response: &CompletionResponse) {
/// let _result = codetether_cognition_core::tool_routing::has_tool_calls(response);
/// # }
/// ```
pub fn has_tool_calls(response: &CompletionResponse) -> bool {
    response
        .message
        .content
        .iter()
        .any(|p| matches!(p, ContentPart::ToolCall { .. }))
}

/// Concatenate the assistant's visible text parts.
///
/// # Arguments
/// * `response` - Provider completion response.
///
/// # Returns
/// The formatted or selected result described above.
///
/// # Examples
/// ```
/// # use codetether_provider_core::{ToolDefinition, CompletionResponse};
/// # fn example(response: &CompletionResponse) {
/// let _result = codetether_cognition_core::tool_routing::assistant_text(response);
/// # }
/// ```
pub fn assistant_text(response: &CompletionResponse) -> String {
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
