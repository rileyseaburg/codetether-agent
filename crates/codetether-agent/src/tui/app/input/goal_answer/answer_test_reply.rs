//! Deterministic no-tool answer for mocked local delivery tests.

use crate::provider::{CompletionResponse, ContentPart, FinishReason, Message, Role, Usage};

pub(super) fn response() -> CompletionResponse {
    CompletionResponse {
        message: Message {
            role: Role::Assistant,
            content: vec![ContentPart::Text {
                text: "Fixture answer".into(),
            }],
        },
        usage: Usage::default(),
        finish_reason: FinishReason::Stop,
    }
}
