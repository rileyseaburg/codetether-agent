//! Completion-response assembly from retained output-item checkpoints.

use codetether_provider_core::{
    CompletionResponse, ContentPart, FinishReason, Message, Role, Usage,
};

pub(super) fn message(content: Vec<ContentPart>) -> Message {
    Message {
        role: Role::Assistant,
        content,
    }
}

pub(super) fn usage(outcome: &codetether_stream_outcome::DrainOutcome) -> Usage {
    outcome
        .response
        .as_ref()
        .map(|value| value.usage.clone())
        .unwrap_or_default()
}

pub(super) fn is_clean_empty(outcome: &codetether_stream_outcome::DrainOutcome) -> bool {
    matches!(outcome.stop, codetether_stream_outcome::StreamStop::Clean)
        && outcome
            .response
            .as_ref()
            .is_none_or(|value| value.message.content.is_empty())
}

pub(super) fn build(content: Vec<ContentPart>, usage: Usage) -> CompletionResponse {
    let tools = content
        .iter()
        .any(|part| matches!(part, ContentPart::ToolCall { .. }));
    CompletionResponse {
        message: message(content),
        usage,
        finish_reason: if tools {
            FinishReason::ToolCalls
        } else {
            FinishReason::Stop
        },
    }
}
