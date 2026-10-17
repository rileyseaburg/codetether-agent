//! Avatar schema property descriptors.
use serde_json::{Value, json};
pub(super) fn model_video() -> Value {
    json!({
        "type": "string",
        "description": "Path to model video for lip-sync (optional, uses default)"
    })
}
pub(super) fn voice_id() -> Value {
    json!({
        "type": "string",
        "description": "Voice ID for TTS (default: Riley 960f89fc)"
    })
}
pub(super) fn podcast_id() -> Value {
    json!({
        "type": "string",
        "description": "Podcast ID (for 'generate_from_episode')"
    })
}
pub(super) fn episode_id() -> Value {
    json!({
        "type": "string",
        "description": "Episode ID (for 'generate_from_episode')"
    })
}
