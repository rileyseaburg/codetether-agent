//! Tool-result message conversion.
use crate::*;
pub(super) fn convert(msg: &Message, role: &str) -> Value {
    if let Some(ContentPart::ToolResult {
        tool_call_id,
        content,
    }) = msg.content.first()
    {
        json!({
            "role": "tool",
            "tool_call_id": tool_call_id,
            "content": content
        })
    } else {
        json!({"role": role, "content": ""})
    }
}
