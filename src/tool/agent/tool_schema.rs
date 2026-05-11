//! JSON schema helpers for the agent tool.
//!
//! Keeps the schema payload out of the tool implementation file.

use serde_json::{Value, json};

/// Returns the JSON schema for the sub-agent management tool.
pub(super) fn agent_tool_parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "action": { "type": "string", "enum": ["spawn", "message", "list", "kill"] },
            "name": { "type": "string", "description": "Agent name" },
            "instructions": { "type": "string", "description": "System instructions (spawn)" },
            "message": { "type": "string", "description": "Message to send" },
            "model": { "type": "string", "description": "Model (spawn). Must be free/subscription-eligible." },
            "ephemeral": { "type": "boolean", "description": "Spawn without durable session persistence; returns an explicit warning." }
        },
        "required": ["action"]
    })
}
