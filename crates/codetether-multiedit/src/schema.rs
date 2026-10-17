//! Multi-edit argument schema.
use serde_json::{Value, json};
pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "edits": {
                "type": "array",
                "description": "Array of edit operations to apply atomically",
                "items": {
                    "type": "object",
                    "properties": {
                        "file": {
                            "type": "string",
                            "description": "Path to the file to edit"
                        },
                        "old_string": {
                            "type": "string",
                            "description": "The exact string to find and replace (must appear exactly once)"
                        },
                        "new_string": {
                            "type": "string",
                            "description": "The replacement string"
                        },
                        "instruction": {
                            "type": "string",
                            "description": "Optional Morph instruction for this edit."
                        },
                        "update": {
                            "type": "string",
                            "description": "Optional Morph update snippet for this edit."
                        }
                    },
                    "required": ["file"]
                }
            }
        },
        "required": ["edits"]
    })
}
