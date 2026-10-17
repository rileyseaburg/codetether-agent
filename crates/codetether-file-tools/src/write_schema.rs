//! write schema for core file tools.
use super::*;

pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "path": {
                "type": "string",
                "description": "The path to the file to write"
            },
            "content": {
                "type": "string",
                "description": "The content to write to the file"
            }
        },
        "required": ["path", "content"],
        "example": {
            "path": "src/config.rs",
            "content": "// Configuration module\n\npub struct Config {\n    pub debug: bool,\n}\n"
        }
    })
}
