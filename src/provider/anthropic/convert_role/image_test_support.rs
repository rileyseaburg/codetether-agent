//! Fixtures for image-bearing Anthropic messages.
use crate::provider::{ContentPart, Message, Role};

pub(super) fn image(data: &str) -> ContentPart {
    ContentPart::Image {
        url: format!("data:image/png;base64,{data}"),
        mime_type: None,
    }
}
pub(super) fn result(id: &str, text: &str) -> ContentPart {
    ContentPart::ToolResult {
        tool_call_id: id.into(),
        content: text.into(),
    }
}
pub(super) fn calls() -> Message {
    Message {
        role: Role::Assistant,
        content: ["a", "b"]
            .into_iter()
            .map(|id| ContentPart::ToolCall {
                id: id.into(),
                name: "screenshot".into(),
                arguments: "{}".into(),
                thought_signature: None,
            })
            .collect(),
    }
}
pub(super) fn transcript() -> Vec<Message> {
    vec![
        calls(),
        Message {
            role: Role::Tool,
            content: vec![
                result("a", "first image"),
                image("YQ=="),
                result("b", "Error: second capture partial"),
                image("Yg=="),
            ],
        },
    ]
}
pub(super) fn convert(messages: &[Message]) -> serde_json::Value {
    let (_, messages) = super::super::convert::messages(messages, false);
    // Round-trip the exact request JSON rather than inspecting generic parts.
    serde_json::from_str(&serde_json::to_string(&messages).unwrap()).unwrap()
}
