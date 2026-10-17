//! Confirmation-edit argument schema.
use serde_json::{Value, json};
pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "path": {
                "type": "string",
                "description": "The path to the file to edit"
            },
            "old_string": {
                "type": "string",
                "description": "The exact string to replace"
            },
            "new_string": {
                "type": "string",
                "description": "The string to replace with"
            },
            "confirm": {
                "type": "boolean",
                "description": "Set to true to confirm and apply changes, false to reject",
                "default": null
            }
        },
        "required": ["path", "old_string", "new_string"]
    })
}
