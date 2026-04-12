//! JSON schema helpers for the agent tool.
//!
//! This module keeps the schema payload out of the tool implementation file so
//! the public tool wrapper stays within the repo's file-size limits.
//!
//! # Examples
//!
//! ```ignore
//! let schema = agent_tool_parameters();
//! ```

use serde_json::{Value, json};

/// Returns the JSON schema for the sub-agent management tool.
///
/// # Examples
///
/// ```ignore
/// let schema = agent_tool_parameters();
/// ```
pub(super) fn agent_tool_parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "action": { "type": "string", "enum": ["spawn", "message", "list", "kill"] },
            "name": { "type": "string", "description": "Agent name" },
            "instructions": { "type": "string", "description": "System instructions (spawn)" },
            "message": { "type": "string", "description": "Message to send" },
            "model": { "type": "string", "description": "Model (spawn). Must be free/subscription-eligible." }
        },
        "required": ["action"]
    })
}
