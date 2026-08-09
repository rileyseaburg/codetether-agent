//! Request/part builders for the Converse body pairing tests.

use crate::provider::{CompletionRequest, ContentPart, Message};

pub(super) fn request(messages: Vec<Message>) -> CompletionRequest {
    CompletionRequest {
        model: "claude-opus-4-7".into(),
        messages,
        tools: vec![],
        temperature: None,
        top_p: None,
        max_tokens: Some(256),
        stop: vec![],
    }
}

pub(super) fn call(id: &str) -> ContentPart {
    ContentPart::ToolCall {
        id: id.into(),
        name: "exec_command".into(),
        arguments: "{}".into(),
        thought_signature: None,
    }
}
