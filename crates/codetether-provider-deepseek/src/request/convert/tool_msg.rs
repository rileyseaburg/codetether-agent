//! tool msg conversion.
use super::*;

pub(super) fn tool_msg(m: &Message) -> Value {
    match m.content.first() {
        Some(ContentPart::ToolResult {
            tool_call_id,
            content,
        }) => json!({"role": "tool", "tool_call_id": tool_call_id, "content": content}),
        _ => json!({"role": "tool", "content": ""}),
    }
}
