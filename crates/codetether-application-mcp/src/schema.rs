//! Model-facing schema for the application MCP connection.

use serde_json::{Value, json};

pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "action": {
                "type": "string",
                "enum": ["list_tools", "call_tool"],
                "description": "Discover tools or call one tool on this connected MCP server."
            },
            "tool_name": {
                "type": "string",
                "description": "MCP tool name; required for call_tool."
            },
            "arguments": {
                "type": "object",
                "description": "Arguments matching the selected MCP tool input schema."
            }
        },
        "required": ["action"],
        "additionalProperties": false
    })
}
