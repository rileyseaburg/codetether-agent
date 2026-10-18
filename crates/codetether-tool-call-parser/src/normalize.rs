//! Recover text calls only when the provider supplied no native tool calls.
use codetether_provider_core::{CompletionResponse, ContentPart, FinishReason, ToolDefinition};

/// # Arguments
/// Receives a provider response and the registered tool definitions.
/// # Returns
/// A response containing recovered calls only when no native calls existed.
/// # Examples
/// ```
/// # fn example(response: codetether_provider_core::CompletionResponse) {
/// let normalized = codetether_tool_call_parser::normalize_textual_tool_calls(response.clone(), &[]);
/// assert_eq!(normalized.message.content.len(), response.message.content.len());
/// # }
/// ```
pub fn normalize_textual_tool_calls(
    mut response: CompletionResponse,
    tools: &[ToolDefinition],
) -> CompletionResponse {
    if response
        .message
        .content
        .iter()
        .any(|p| matches!(p, ContentPart::ToolCall { .. }))
    {
        return response;
    }

    if tools.is_empty() {
        return response;
    }

    let mut rewritten = Vec::with_capacity(response.message.content.len());
    let mut parsed_calls: Vec<(String, String)> = Vec::new();
    let allowed_tools: std::collections::HashSet<&str> =
        tools.iter().map(|t| t.name.as_str()).collect();

    for part in response.message.content {
        match part {
            ContentPart::Text { text } => {
                super::text::append(&text, &allowed_tools, &mut rewritten, &mut parsed_calls);
            }
            other => rewritten.push(other),
        }
    }

    if parsed_calls.is_empty() {
        response.message.content = rewritten;
        return response;
    }

    super::call_parts::append(&mut rewritten, parsed_calls);

    response.message.content = rewritten;
    response.finish_reason = FinishReason::ToolCalls;
    response
}
