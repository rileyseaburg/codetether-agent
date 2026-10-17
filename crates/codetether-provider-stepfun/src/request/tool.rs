//! tool message conversion.
use crate::*;
pub(super) fn push(msg: &Message, result: &mut Vec<ChatMessage>) {
    for part in &msg.content {
        if let ContentPart::ToolResult {
            tool_call_id,
            content,
        } = part
        {
            result.push(ChatMessage {
                role: "tool".to_string(),
                content: Some(content.clone()),
                tool_calls: None,
                tool_call_id: Some(tool_call_id.clone()),
            });
        }
    }
}
