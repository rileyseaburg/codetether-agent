//! Provider message fixtures for page policy tests.
use super::*;

pub(super) fn user(s: &str) -> Message {
    Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: s.to_string(),
        }],
    }
}

pub(super) fn assistant(s: &str) -> Message {
    Message {
        role: Role::Assistant,
        content: vec![ContentPart::Text {
            text: s.to_string(),
        }],
    }
}

pub(super) fn tool_result(id: &str, body: &str) -> Message {
    Message {
        role: Role::Tool,
        content: vec![ContentPart::ToolResult {
            tool_call_id: id.to_string(),
            content: body.to_string(),
        }],
    }
}
