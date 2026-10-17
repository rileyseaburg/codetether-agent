//! Tool-reply selection for the existing history conventions.
use super::HistoryFormat;
use crate::{ContentPart, Message};
use serde_json::{Value, json};
pub(super) fn convert(message: &Message, format: HistoryFormat) -> Vec<Value> {
    match format {
        HistoryFormat::SingleReply => vec![match message.content.first() {
            Some(ContentPart::ToolResult {
                tool_call_id,
                content,
            }) => {
                json!({"role": "tool", "tool_call_id": tool_call_id, "content": content})
            }
            _ => json!({"role": "tool", "content": ""}),
        }],
        HistoryFormat::ExpandedReplies => message
            .content
            .iter()
            .filter_map(|part| match part {
                ContentPart::ToolResult {
                    tool_call_id,
                    content,
                } => {
                    Some(json!({"role": "tool", "tool_call_id": tool_call_id, "content": content}))
                }
                _ => None,
            })
            .collect(),
    }
}
