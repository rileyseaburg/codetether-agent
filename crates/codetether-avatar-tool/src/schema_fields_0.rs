//! Avatar schema property descriptors.
use serde_json::{Value, json};
pub(super) fn action() -> Value {
    json!({
        "type": "string",
        "enum": ["status", "start", "stop", "generate", "generate_from_episode", "list_models", "upload_model"],
        "description": "Action to perform"
    })
}
pub(super) fn text() -> Value {
    json!({
        "type": "string",
        "description": "Text to convert to speech then avatar video (for 'generate')"
    })
}
pub(super) fn audio_url() -> Value {
    json!({
        "type": "string",
        "description": "Path to audio file on server (for 'generate')"
    })
}
pub(super) fn audio_file() -> Value {
    json!({
        "type": "string",
        "description": "Local path to audio file to upload (for 'generate')"
    })
}
