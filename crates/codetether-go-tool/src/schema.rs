//! JSON schema for autonomous pipeline actions.

use serde_json::{Value, json};
pub(crate) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "action": {
                "type": "string",
                "enum": ["execute", "watch", "status"],
                "description": "Action to perform. Use 'execute' to start, 'watch' to monitor progress, 'status' for OKR results."
            },
            "task": {
                "type": "string",
                "description": "Task description for autonomous execution"
            },
            "max_iterations": {
                "type": "integer",
                "description": "Maximum Ralph iterations (default: 10)"
            },
            "max_concurrent_stories": {
                "type": "integer",
                "description": "Maximum concurrent stories in Ralph (default: 3)"
            },
            "model": {
                "type": "string",
                "description": "Model to use for PRD generation and Ralph execution"
            },
            "okr_id": {
                "type": "string",
                "description": "OKR ID for watch/status actions"
            }
        },
        "required": ["action"]
    })
}
