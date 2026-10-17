//! tool result policy.
use super::*;

pub(super) fn tool_result(body: &str) -> Message {
    Message {
        role: Role::Tool,
        content: vec![ContentPart::ToolResult {
            tool_call_id: "call-1".to_string(),
            content: body.to_string(),
        }],
    }
}
