use super::{ContentPart, Message, Role};
pub(super) const PIXELS: &str = "data:image/png;base64,aW1hZ2U=";
pub(super) fn image() -> ContentPart {
    ContentPart::Image {
        url: PIXELS.into(),
        mime_type: Some("image/png".into()),
    }
}
pub(super) fn text(value: &str) -> ContentPart {
    ContentPart::Text { text: value.into() }
}
pub(super) fn message(role: Role, content: Vec<ContentPart>) -> Message {
    Message { role, content }
}
pub(super) fn result(id: &str) -> Message {
    message(
        Role::Tool,
        vec![
            ContentPart::ToolResult {
                tool_call_id: id.into(),
                content: format!("result {id}"),
            },
            image(),
        ],
    )
}
