//! Shared completion inputs for HTTP contract tests.

use super::*;

pub(super) fn request(model: &str) -> CompletionRequest {
    CompletionRequest {
        model: model.into(),
        messages: vec![Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "hello".into(),
            }],
        }],
        tools: Vec::new(),
        temperature: Some(0.7),
        top_p: Some(0.8),
        max_tokens: Some(9),
        stop: Vec::new(),
    }
}
