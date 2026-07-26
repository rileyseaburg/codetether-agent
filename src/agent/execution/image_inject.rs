//! Convert tool-returned image metadata into vision content.

use crate::provider::ContentPart;
use crate::tool::ToolResult;

pub(super) fn tool_image(result: &ToolResult) -> Option<ContentPart> {
    let Some(img) = result.metadata.get("image_data_url") else {
        return None;
    };
    let (Some(url), Some(mime)) = (
        img.get("data_url").and_then(|v| v.as_str()),
        img.get("mime_type").and_then(|v| v.as_str()),
    ) else {
        return None;
    };
    Some(ContentPart::Image {
        url: url.to_string(),
        mime_type: Some(mime.to_string()),
    })
}
