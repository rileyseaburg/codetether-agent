//! Action and privacy schema definitions.
use serde_json::{Value, json};
pub(super) fn action() -> Value {
    json!({
        "type": "string",
        "enum": ["publish_episode", "upload_video", "status"],
        "description": "Action to perform"
    })
}
pub(super) fn privacy_status() -> Value {
    json!({
        "type": "string",
        "enum": ["public", "unlisted", "private"],
        "description": "Video privacy (default: unlisted)",
        "default": "unlisted"
    })
}
