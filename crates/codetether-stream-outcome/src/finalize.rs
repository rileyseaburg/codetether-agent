//! Assemble content in provider order and derive the final finish reason.
use crate::ToolAccumulator;
use codetether_provider_core::{ContentPart, FinishReason, Message, Role, Usage};

/// Build the final [`CompletionResponse`](codetether_provider_core::CompletionResponse)
/// from accumulated thinking, text, and tool calls.
/// # Arguments
/// Receives thinking text/signature, visible text, tools and final usage.
/// # Returns
/// An assistant response with thinking first, then text, then ordered tool calls.
/// # Examples
/// ```
/// let response = codetether_stream_outcome::build_response(String::new(), None, "ok".into(), vec![], Default::default());
/// assert_eq!(response.message.content.len(), 1);
/// ```
pub fn build_response(
    thinking: String,
    reasoning_signature: Option<String>,
    text: String,
    tools: Vec<ToolAccumulator>,
    usage: Usage,
) -> codetether_provider_core::CompletionResponse {
    let mut content = Vec::new();
    if !thinking.is_empty() || reasoning_signature.is_some() {
        content.push(ContentPart::Thinking {
            text: thinking,
            signature: reasoning_signature,
        });
    }
    if !text.is_empty() {
        content.push(ContentPart::Text { text });
    }
    for tool in tools {
        content.push(ContentPart::ToolCall {
            id: tool.id,
            name: tool.name,
            arguments: tool.arguments,
            thought_signature: None,
        });
    }

    let finish_reason = if content
        .iter()
        .any(|part| matches!(part, ContentPart::ToolCall { .. }))
    {
        FinishReason::ToolCalls
    } else {
        FinishReason::Stop
    };

    codetether_provider_core::CompletionResponse {
        message: Message {
            role: Role::Assistant,
            content,
        },
        usage,
        finish_reason,
    }
}
