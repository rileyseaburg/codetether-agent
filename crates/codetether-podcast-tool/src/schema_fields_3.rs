//! Podcast schema property descriptors.
use serde_json::{Value, json};
pub(super) fn episode_number() -> Value {
    json!({
        "type": "integer",
        "description": "Episode number"
    })
}
pub(super) fn season_number() -> Value {
    json!({
        "type": "integer",
        "description": "Season number"
    })
}
