//! Browse action and absolute turn-index schema.
use serde_json::{Value, json};
pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "action": {
                "type": "string",
                "enum": ["list", "list_turns", "show", "show_turn"],
                "description": "Operation: 'list' enumerates turn paths, \
                                'show_turn' returns one turn body."
            },
            "turn": {
                "type": "integer",
                "description": "Zero-based turn index, required with \
                                action='show_turn'.",
                "minimum": 0
            }
        },
        "required": []
    })
}
