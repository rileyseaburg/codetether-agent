//! info schema for filesystem inspection.
use super::*;
pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "path": {
                "type": "string",
                "description": "The path to the file to inspect"
            }
        },
        "required": ["path"],
        "example": {
            "path": "src/main.rs"
        }
    })
}
