//! Avatar schema property descriptors.
use serde_json::{Value, json};
pub(super) fn upload_youtube() -> Value {
    json!({
        "type": "boolean",
        "description": "Upload generated video to YouTube (for 'generate_from_episode')",
        "default": false
    })
}
pub(super) fn privacy_status() -> Value {
    json!({
        "type": "string",
        "enum": ["public", "unlisted", "private"],
        "description": "YouTube privacy status (default: unlisted)"
    })
}
pub(super) fn name() -> Value {
    json!({
        "type": "string",
        "description": "Model name (for 'upload_model')"
    })
}
pub(super) fn file_path() -> Value {
    json!({
        "type": "string",
        "description": "Path to video file (for 'upload_model')"
    })
}
