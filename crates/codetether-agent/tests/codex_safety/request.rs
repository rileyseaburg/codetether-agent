//! Supported request fixture that reaches the local backend safety gate.

use codetether_agent::provider::{CompletionRequest, ContentPart, Message, Role};

pub(super) fn request() -> CompletionRequest {
    CompletionRequest {
        messages: vec![Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "hello".to_string(),
            }],
        }],
        tools: vec![],
        // Unsupported models fail validation before the backend opt-in gate.
        model: "gpt-5.5".to_string(),
        temperature: None,
        top_p: None,
        max_tokens: None,
        stop: vec![],
    }
}
