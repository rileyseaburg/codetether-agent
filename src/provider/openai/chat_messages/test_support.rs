//! Shared serialization fixtures for both chat transport paths.

use crate::provider::{ContentPart, Message};
use serde_json::Value;

pub(super) fn serialized(messages: &[Message]) -> [Vec<Value>; 2] {
    let sdk = super::convert(messages)
        .unwrap()
        .into_iter()
        .map(|message| serde_json::to_value(message).unwrap())
        .collect();
    [
        sdk,
        crate::provider::openai::sse_msg::messages_json(messages),
    ]
}

pub(super) fn image() -> ContentPart {
    ContentPart::Image {
        url: "data:image/png;base64,iVBORw0KGgo=".into(),
        mime_type: Some("image/png".into()),
    }
}

pub(super) fn text(value: &str) -> ContentPart {
    ContentPart::Text { text: value.into() }
}

pub(super) fn reply(id: &str) -> ContentPart {
    ContentPart::ToolResult {
        tool_call_id: id.into(),
        content: format!("output {id}"),
    }
}
