//! user content conversion.
use crate::*;
pub(super) fn push(msg: &Message, _system_blocks: &mut Vec<Value>, api_messages: &mut Vec<Value>) {
    let mut content_parts = image_content::user::parts(msg);
    if content_parts.is_empty() {
        content_parts.push(json!({"type": "text", "text": " "}));
    }
    api_messages.push(json!({
        "role": "user",
        "content": content_parts
    }));
}
