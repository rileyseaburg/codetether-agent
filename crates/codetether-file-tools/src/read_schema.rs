//! read schema for core file tools.
use super::*;

pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "path": {
                "type": "string",
                "description": "The path to the file to read"
            },
            "offset": {
                "type": "integer",
                "description": "Line number to start reading from (1-indexed)"
            },
            "limit": {
                "type": "integer",
                "description": "Maximum number of lines to read"
            }
        },
        "required": ["path"],
        "example": {
            "path": "src/main.rs",
            "offset": 1,
            "limit": 100
        }
    })
}
