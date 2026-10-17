//! Podcast schema property descriptors.
use serde_json::{Value, json};
pub(super) fn description() -> Value {
    json!({
        "type": "string",
        "description": "Description for podcast or episode"
    })
}
pub(super) fn author() -> Value {
    json!({
        "type": "string",
        "description": "Podcast author name (default: CodeTether Agent)"
    })
}
pub(super) fn email() -> Value {
    json!({
        "type": "string",
        "description": "Contact email for podcast"
    })
}
pub(super) fn category() -> Value {
    json!({
        "type": "string",
        "description": "Podcast category (default: Technology)"
    })
}
