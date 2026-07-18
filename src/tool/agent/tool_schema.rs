//! JSON schema helpers for the agent tool.
//!
//! Keeps the schema payload out of the tool implementation file.

use serde_json::{Value, json};

/// Returns the JSON schema for the sub-agent management tool.
pub(super) fn agent_tool_parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "action": { "type": "string", "enum": ["spawn", "message", "list", "status", "interrupt", "close", "resume", "kill"], "description": "Agent-instance action. `list` shows spawned agent instances, not available providers or models (use `codetether models --json`). `interrupt` stops one turn, `close` frees an active slot, `resume` reopens the same durable child, and `kill` removes its registration." },
            "name": { "type": "string", "description": "Local agent name or durable child session ID" },
            "instructions": { "type": "string", "description": "System instructions (spawn)" },
            "message": { "type": "string", "description": "Message to send" },
            "model": { "type": "string", "description": "Model (spawn). Should be free/subscription-eligible; otherwise a cost warning is returned." },
            "ephemeral": { "type": "boolean", "description": "Run once without a child transcript or registry entry. Ephemeral runs are synchronous." },
            "detach": { "type": "boolean", "description": "Defaults to false so the caller receives the result. Interactive clients may set true for background execution. Ephemeral agents cannot detach." }
        },
        "required": ["action"]
    })
}

#[cfg(test)]
#[path = "tool_schema_tests.rs"]
mod tests;
