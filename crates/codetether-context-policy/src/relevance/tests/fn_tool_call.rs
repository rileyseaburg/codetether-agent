//! tool call policy.
use super::*;

pub(super) fn tool_call(name: &str) -> Message {
    Message {
        role: Role::Assistant,
        content: vec![ContentPart::ToolCall {
            id: "call-1".to_string(),
            name: name.to_string(),
            arguments: "{}".to_string(),
            thought_signature: None,
        }],
    }
}
