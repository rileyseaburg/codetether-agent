use super::{ContentPart, Message, parts};
use serde_json::{Value, json};

// Images belong to the nearest preceding ToolResult in the stored message.
// Explicit leading-image convention: before the first result, attach only to
// that first result, never to every call. With no result, label the missing ID
// rather than inventing one. Each image is emitted exactly once.
pub(super) fn companions(message: &Message) -> Vec<Value> {
    let mut call_id = message.content.iter().find_map(|part| match part {
        ContentPart::ToolResult { tool_call_id, .. } => Some(tool_call_id.as_str()),
        _ => None,
    });
    let mut output = Vec::new();
    for part in &message.content {
        match part {
            ContentPart::ToolResult { tool_call_id, .. } => call_id = Some(tool_call_id),
            ContentPart::Image { url, .. } => {
                let label = match call_id {
                    Some(id) => format!("Image from tool result: {id}"),
                    None => "Image from tool output (missing tool_call_id)".into(),
                };
                output.push(json!({"role": "user", "content": [
                    {"type": "text", "text": label}, parts::image(url)
                ]}));
            }
            _ => {}
        }
    }
    output
}
