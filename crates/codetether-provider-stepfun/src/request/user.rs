//! user message conversion.
use crate::*;
pub(super) fn push(msg: &Message, result: &mut Vec<ChatMessage>) {
    let content = super::text::content(msg);
    result.push(ChatMessage {
        role: "user".to_string(),
        content: Some(content),
        tool_calls: None,
        tool_call_id: None,
    });
}
