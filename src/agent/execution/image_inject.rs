//! Inject tool-returned images as vision content parts.

use crate::provider::{ContentPart, Message, Role};
use crate::session::Session;
use crate::tool::ToolResult;

/// If `result.metadata` carries `image_data_url`, inject a user message
/// with a `ContentPart::Image` so the provider can render it as vision.
pub fn inject_tool_image(session: &mut Session, result: &ToolResult) {
    let Some(img) = result.metadata.get("image_data_url") else {
        return;
    };
    let (Some(url), Some(mime)) = (
        img.get("data_url").and_then(|v| v.as_str()),
        img.get("mime_type").and_then(|v| v.as_str()),
    ) else {
        return;
    };
    session.add_message(Message {
        role: Role::User,
        content: vec![ContentPart::Image {
            url: url.to_string(),
            mime_type: Some(mime.to_string()),
        }],
    });
}
