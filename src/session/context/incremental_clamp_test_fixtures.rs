//! Message fixtures for incremental overflow-clamp regression tests.

use crate::provider::{ContentPart, Message, Role};

fn message(role: Role, content: ContentPart) -> Message {
    Message {
        role,
        content: vec![content],
    }
}

fn text(role: Role, value: &str) -> Message {
    message(role, ContentPart::Text { text: value.into() })
}

pub(super) fn active_tool_turn() -> Vec<Message> {
    vec![
        text(Role::Assistant, "stale"),
        text(Role::User, "inspect URL"),
        message(
            Role::Assistant,
            ContentPart::ToolCall {
                id: "call-1".into(),
                name: "browserctl".into(),
                arguments: "{}".into(),
                thought_signature: None,
            },
        ),
        message(
            Role::Tool,
            ContentPart::ToolResult {
                tool_call_id: "call-1".into(),
                content: "ok".into(),
            },
        ),
    ]
}
