//! Input fixtures and JSON round trips for Vertex conversion regressions.
use crate::provider::{ContentPart, Message, Role};
use serde_json::{Value, json};

pub(super) fn message(role: Role, content: Vec<ContentPart>) -> Message {
    Message { role, content }
}
pub(super) fn image(url: &str) -> ContentPart {
    ContentPart::Image {
        url: url.into(),
        mime_type: None,
    }
}
pub(super) fn text(value: &str) -> ContentPart {
    ContentPart::Text { text: value.into() }
}
pub(super) fn result(id: &str, value: &str) -> ContentPart {
    ContentPart::ToolResult {
        tool_call_id: id.into(),
        content: value.into(),
    }
}
pub(super) fn call(id: &str) -> ContentPart {
    ContentPart::ToolCall {
        id: id.into(),
        name: "screenshot".into(),
        arguments: "{}".into(),
        thought_signature: None,
    }
}
pub(super) fn convert(messages: &[Message]) -> Value {
    let (system, messages) = super::super::VertexAnthropicProvider::convert_messages(messages);
    let wire = json!({"system": system, "messages": messages});
    serde_json::from_str(&serde_json::to_string(&wire).unwrap()).unwrap()
}
