//! Completion-response assembly from retained output-item checkpoints.

use crate::provider::{CompletionResponse, ContentPart, FinishReason, Message, Role, Usage};

pub(super) fn message(content: Vec<ContentPart>) -> Message {
    Message {
        role: Role::Assistant,
        content,
    }
}

pub(super) fn usage(outcome: &super::super::super::outcome::DrainOutcome) -> Usage {
    outcome
        .response
        .as_ref()
        .map(|value| value.usage.clone())
        .unwrap_or_default()
}

pub(super) fn is_clean_empty(outcome: &super::super::super::outcome::DrainOutcome) -> bool {
    matches!(
        outcome.stop,
        super::super::super::outcome::StreamStop::Clean
    ) && outcome
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

pub(super) fn prepend(
    mut response: CompletionResponse,
    mut content: Vec<ContentPart>,
) -> CompletionResponse {
    content.append(&mut response.message.content);
    response.message.content = content;
    response
}
