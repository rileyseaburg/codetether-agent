//! Argument schema for grouped edit confirmation.
use serde_json::{Value, json};
pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "edits": {
                "type": "array",
                "description": "Array of edit operations to preview",
                "items": {
                    "type": "object",
                    "properties": {
                        "file": {
                            "type": "string",
                            "description": "Path to the file to edit"
                        },
                        "old_string": {
                            "type": "string",
                            "description": "The exact string to find and replace"
                        },
                        "new_string": {
                            "type": "string",
                            "description": "The string to replace it with"
                        }
                    },
                    "required": ["file", "old_string", "new_string"]
                }
            },
            "confirm": {
                "type": "boolean",
                "description": "Set to true to confirm and apply all changes, false to reject all",
                "default": null
            }
        },
        "required": ["edits"]
    })
}
