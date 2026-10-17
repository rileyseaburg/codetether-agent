//! Packed tool-result serialization.
use crate::*;
pub(super) fn convert(msg: &Message) -> Value {
    let mut content_parts: Vec<Value> = Vec::new();
    let mut tool_call_id = None;
    for part in &msg.content {
        match part {
            ContentPart::ToolResult {
                tool_call_id: id,
                content,
            } => {
                tool_call_id = Some(id.clone());
                content_parts.push(json!(content));
            }
            ContentPart::Text { text } => {
                content_parts.push(json!(text));
            }
            _ => {}
        }
    }
    let content_str = content_parts
        .iter()
        .filter_map(|v| v.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    let mut m = json!({
        "role": "tool",
        "content": content_str,
    });
    if let Some(id) = tool_call_id {
        m["tool_call_id"] = json!(id);
    }
    return m;
}
