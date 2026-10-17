//! JSON schema describing undo steps and preview mode.
use serde_json::{Value, json};
pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "steps": {
                "type": "integer",
                "minimum": 1,
                "maximum": 10,
                "description": "Number of commits to undo",
                "default": 1
            },
            "preview": {
                "type": "boolean",
                "description": "Show what would be undone without actually doing it",
                "default": false
            }
        }
    })
}
