//! tool msg.
use super::*;

pub(super) fn tool_msg(id: &str, content: &str) -> Message {
    Message {
        role: codetether_provider_core::Role::Tool,
        content: vec![ContentPart::ToolResult {
            tool_call_id: id.into(),
            content: content.into(),
        }],
    }
}
