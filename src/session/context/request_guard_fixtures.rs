//! Small provider-request fixtures for budget guard tests.
use crate::provider::{CompletionRequest, ContentPart, Message, Role};

pub(super) fn text(role: Role, text: &str) -> Message {
    Message {
        role,
        content: vec![ContentPart::Text { text: text.into() }],
    }
}

pub(super) fn request() -> CompletionRequest {
    CompletionRequest {
        messages: vec![
            text(Role::System, "stable instructions"),
            text(Role::User, "previous substantive user request"),
            text(Role::Assistant, "stable previous response"),
            text(Role::User, "current substantive user request"),
        ],
        tools: vec![],
        model: "mistral".into(),
        temperature: None,
        top_p: None,
        max_tokens: Some(8192),
        stop: vec![],
    }
}
