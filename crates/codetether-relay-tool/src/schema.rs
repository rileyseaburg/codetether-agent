//! schema for relay coordination.
use super::*;

pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "action": {
                "type": "string",
                "enum": ["delegate", "handoff", "status", "list_agents", "init", "complete"],
                "description": "Action to perform"
            },
            "target_agent": {
                "type": "string",
                "description": "Target agent name for delegation/handoff"
            },
            "message": {
                "type": "string",
                "description": "Message to send to the target agent"
            },
            "context": {
                "type": "object",
                "description": "Additional context to pass along (JSON object)"
            },
            "relay_id": {
                "type": "string",
                "description": "Relay ID to use (auto-generated if not provided)"
            },
            "okr_id": {
                "type": "string",
                "description": "Optional OKR ID to associate with this relay"
            },
            "task": {
                "type": "string",
                "description": "Task description for initializing a new relay"
            }
        },
        "required": ["action"]
    })
}
