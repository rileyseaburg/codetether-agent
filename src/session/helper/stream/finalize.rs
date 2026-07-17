//! Final assembly of a streamed completion into a
//! [`CompletionResponse`](crate::provider::CompletionResponse).
//!
//! Keeps [`super::collect_stream_completion_with_events`] focused on chunk
//! collection while this module owns content ordering and finish-reason
//! derivation. Thinking text is emitted first (mirroring provider output
//! order) so a thinking-only turn still yields non-empty assistant content
//! instead of a silent empty message.

use crate::provider::{ContentPart, FinishReason, Message, Role, Usage};

/// Accumulates one tool call's identity and argument deltas during streaming.
#[derive(Default)]
pub(super) struct ToolAccumulator {
    pub(super) id: String,
    pub(super) name: String,
    pub(super) arguments: String,
}

/// Build the final [`CompletionResponse`](crate::provider::CompletionResponse)
/// from accumulated thinking, text, and tool calls.
pub(super) fn build_response(
    thinking: String,
    reasoning_signature: Option<String>,
    text: String,
    tools: Vec<ToolAccumulator>,
    usage: Usage,
) -> crate::provider::CompletionResponse {
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

    crate::provider::CompletionResponse {
        message: Message {
            role: Role::Assistant,
            content,
        },
        usage,
        finish_reason,
    }
}
