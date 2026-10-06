//! Request identity regression tests.
use super::inject;
use crate::provider::{CompletionRequest, ContentPart, Message, Role};

#[path = "tests/dispatch.rs"]
mod dispatch;
#[path = "tests/preservation.rs"]
mod preservation;
#[path = "tests/switching.rs"]
mod switching;

fn message(role: Role, text: &str) -> Message {
    Message {
        role,
        content: vec![ContentPart::Text { text: text.into() }],
    }
}

fn request(model: &str) -> CompletionRequest {
    CompletionRequest {
        messages: vec![
            message(Role::System, "Preserve instructions"),
            message(Role::User, "Who are you?"),
        ],
        tools: vec![],
        model: model.into(),
        temperature: Some(0.2),
        top_p: Some(0.8),
        max_tokens: Some(123),
        stop: vec!["stop".into()],
    }
}

fn text(message: &Message) -> &str {
    match &message.content[0] {
        ContentPart::Text { text } => text,
        _ => panic!("expected text"),
    }
}
