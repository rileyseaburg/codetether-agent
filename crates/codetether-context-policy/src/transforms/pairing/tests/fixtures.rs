//! Provider message fixtures for pairing regressions.
use super::*;

pub(super) fn call(id: &str) -> Message {
    Message {
        role: Role::Assistant,
        content: vec![ContentPart::ToolCall {
            id: id.into(),
            name: "t".into(),
            arguments: "{}".into(),
            thought_signature: None,
        }],
    }
}

pub(super) fn result(id: &str) -> Message {
    Message {
        role: Role::Tool,
        content: vec![ContentPart::ToolResult {
            tool_call_id: id.into(),
            content: "ok".into(),
        }],
    }
}

pub(super) fn user(t: &str) -> Message {
    Message {
        role: Role::User,
        content: vec![ContentPart::Text { text: t.into() }],
    }
}
