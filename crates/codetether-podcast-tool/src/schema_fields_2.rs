//! Podcast schema property descriptors.
use serde_json::{Value, json};
pub(super) fn subcategory() -> Value {
    json!({
        "type": "string",
        "description": "Podcast subcategory"
    })
}
pub(super) fn language() -> Value {
    json!({
        "type": "string",
        "description": "Podcast language code (default: en-us)"
    })
}
pub(super) fn script() -> Value {
    json!({
        "type": "string",
        "description": "Episode script text to convert to speech (required for create_episode)"
    })
}
pub(super) fn voice_id() -> Value {
    json!({
        "type": "string",
        "description": "Voice profile ID for TTS (default: Riley voice 960f89fc)"
    })
}
