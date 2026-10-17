use super::{ContentPart, Message, Role};
use serde_json::Value;

// Split packed tool messages without losing replies, with or without images.
pub(super) fn convert(
    message: &Message,
    serialize: &impl Fn(&Message) -> Vec<Value>,
) -> Vec<Value> {
    let mut output = Vec::new();
    let mut reply = Message {
        role: Role::Tool,
        content: Vec::new(),
    };
    let mut has_result = false;
    for part in &message.content {
        if matches!(part, ContentPart::Image { .. }) {
            continue;
        }
        if matches!(part, ContentPart::ToolResult { .. }) {
            if has_result {
                output.extend(serialize(&reply));
                reply.content.clear();
            }
            has_result = true;
        }
        reply.content.push(part.clone());
    }
    if !reply.content.is_empty() || output.is_empty() {
        output.extend(serialize(&reply));
    }
    output
}
