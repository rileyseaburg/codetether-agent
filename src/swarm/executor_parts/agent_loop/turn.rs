//! Structured content extracted from one provider response.

use super::state::ToolCall;
use crate::provider::{CompletionResponse, ContentPart, FinishReason, Message};

pub(super) struct Turn {
    pub message: Message,
    pub finish: FinishReason,
    pub text: String,
    pub thinking: String,
    pub tools: Vec<ToolCall>,
}

pub(super) fn parse(response: CompletionResponse) -> Turn {
    let mut text = Vec::new();
    let mut thinking = Vec::new();
    let mut tools = Vec::new();
    for part in &response.message.content {
        match part {
            ContentPart::Text { text: value } => text.push(value.clone()),
            ContentPart::Thinking { text: value, .. } if !value.is_empty() => {
                thinking.push(value.clone())
            }
            ContentPart::ToolCall {
                id,
                name,
                arguments,
                ..
            } => tools.push(ToolCall {
                id: id.clone(),
                name: name.clone(),
                arguments: arguments.clone(),
            }),
            ContentPart::Thinking { .. }
            | ContentPart::ToolResult { .. }
            | ContentPart::Image { .. }
            | ContentPart::File { .. } => {}
        }
    }
    tracing::info!(finish_reason = ?response.finish_reason,
        prompt_tokens = response.usage.prompt_tokens,
        completion_tokens = response.usage.completion_tokens, "Sub-agent provider turn completed");
    Turn {
        message: response.message,
        finish: response.finish_reason,
        text: text.join("\n"),
        thinking: thinking.join("\n"),
        tools,
    }
}
