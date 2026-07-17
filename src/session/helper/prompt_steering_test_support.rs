//! Message helpers for the steering test provider.

use crate::provider::*;

pub(super) fn has_text(request: &CompletionRequest, expected: &str) -> bool {
    request
        .messages
        .iter()
        .flat_map(|message| &message.content)
        .any(|part| matches!(part, ContentPart::Text { text } if text == expected))
}

pub(super) fn response(text: &str) -> CompletionResponse {
    let message = Message {
        role: Role::Assistant,
        content: vec![ContentPart::Text { text: text.into() }],
    };
    CompletionResponse {
        message,
        usage: Usage::default(),
        finish_reason: FinishReason::Stop,
    }
}
