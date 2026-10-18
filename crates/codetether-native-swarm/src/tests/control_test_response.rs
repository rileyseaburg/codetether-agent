//! Small final-message and tool fixtures for delegated control regressions.

use codetether_provider_core::*;

pub(super) fn response(tool: bool) -> CompletionResponse {
    let content = if tool {
        ContentPart::ToolCall {
            id: "never".into(),
            name: "image".into(),
            arguments: "{}".into(),
            thought_signature: None,
        }
    } else {
        ContentPart::Text {
            text: "done".into(),
        }
    };
    CompletionResponse {
        message: Message {
            role: Role::Assistant,
            content: vec![content],
        },
        finish_reason: FinishReason::Stop,
        usage: Usage {
            prompt_tokens: 2,
            completion_tokens: 3,
            ..Usage::default()
        },
    }
}
