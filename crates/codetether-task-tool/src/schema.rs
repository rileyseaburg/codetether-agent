//! schema for in-memory task tracking.
use super::*;

pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "action": {
                "type": "string",
                "enum": ["create", "status", "complete", "list", "cancel"],
                "description": "Action to perform"
            },
            "id": {"type": "string", "description": "Task ID (for status/complete/cancel)"},
            "description": {"type": "string", "description": "Task description (for create)"},
            "result": {"type": "string", "description": "Task result (for complete)"}
        },
        "required": ["action"]
    })
}
