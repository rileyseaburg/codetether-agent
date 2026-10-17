//! list schema for core file tools.
use super::*;

pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "path": {
                "type": "string",
                "description": "The path to the directory to list"
            }
        },
        "required": ["path"],
        "example": {
            "path": "src/"
        }
    })
}
