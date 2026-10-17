//! Pin action and turn-index parameter schema.
use serde_json::{Value, json};
pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "action": { "type": "string", "enum": ["pin", "unpin"],
                "description": "Whether to pin or unpin the turn." },
            "turn_index": { "type": "integer", "minimum": 0,
                "description": "0-based index into the session transcript." }
        },
        "required": ["action", "turn_index"]
    })
}
