//! schema for the token count tool.
use super::*;

pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "paths": {"type": "array", "items": {"type": "string"},
                      "description": "Files to measure"}
        },
        "required": ["paths"]
    })
}
