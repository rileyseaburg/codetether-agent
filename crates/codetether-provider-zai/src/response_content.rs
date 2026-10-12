//! Conversion of a buffered assistant message into protocol content blocks.
use crate::{ContentPart, response_wire::ZaiMessage};
use serde_json::Value;

pub(crate) fn convert(message: ZaiMessage) -> Vec<ContentPart> {
    let mut content = Vec::new();
    if let Some(text) = message.reasoning_content {
        content.push(ContentPart::Thinking {
            text,
            signature: None,
        });
    }
    if let Some(text) = message.content {
        content.push(ContentPart::Text { text });
    }
    for call in message.tool_calls.unwrap_or_default() {
        let arguments = match call.function.arguments {
            Value::String(text) => text,
            other => other.to_string(),
        };
        content.push(ContentPart::ToolCall {
            id: call.id,
            name: call.function.name,
            arguments,
            thought_signature: None,
        });
    }
    content
}
