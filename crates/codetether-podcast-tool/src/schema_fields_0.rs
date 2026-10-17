//! Podcast schema property descriptors.
use serde_json::{Value, json};
pub(super) fn action() -> Value {
    json!({
        "type": "string",
        "enum": ["create_podcast", "create_episode", "list_podcasts", "get_podcast", "delete_podcast", "delete_episode"],
        "description": "Action to perform"
    })
}
pub(super) fn podcast_id() -> Value {
    json!({
        "type": "string",
        "description": "Podcast ID (required for create_episode, get_podcast, delete_podcast, delete_episode)"
    })
}
pub(super) fn episode_id() -> Value {
    json!({
        "type": "string",
        "description": "Episode ID (required for delete_episode)"
    })
}
pub(super) fn title() -> Value {
    json!({
        "type": "string",
        "description": "Title for podcast or episode"
    })
}
