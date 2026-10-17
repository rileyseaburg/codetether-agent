//! tool msg.
use super::*;

pub(super) fn tool_msg(content: &str) -> Message {
    Message {
        role: codetether_provider_core::Role::Tool,
        content: vec![ContentPart::ToolResult {
            tool_call_id: "x".into(),
            content: content.into(),
        }],
    }
}
