//! Tool argument schema.
use serde_json::{Value, json};
pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "filePath": {"type": "string", "description": "Absolute path to file"},
            "oldString": {"type": "string", "description": "Text to replace"},
            "newString": {"type": "string", "description": "Replacement text"},
            "replaceAll": {"type": "boolean", "description": "Replace all occurrences", "default": false}
        },
        "required": ["filePath", "oldString", "newString"]
    })
}
