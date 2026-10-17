//! preview schema for filesystem inspection.
use super::*;
pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "path": {
                "type": "string",
                "description": "The path to the file"
            },
            "head": {
                "type": "integer",
                "description": "Number of lines from the beginning (default: 10)",
                "default": 10
            },
            "tail": {
                "type": "integer",
                "description": "Number of lines from the end (default: 0, set to show tail)",
                "default": 0
            }
        },
        "required": ["path"],
        "example": {
            "path": "src/main.rs",
            "head": 20,
            "tail": 10
        }
    })
}
