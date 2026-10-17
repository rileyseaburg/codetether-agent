//! Bridge action and connection parameter schema.
use serde_json::{Value, json};
pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "action": {
                "type": "string",
                "description": "Action to perform: list_tools, call_tool, list_resources, read_resource",
                "enum": ["list_tools", "call_tool", "list_resources", "read_resource"]
            },
            "command": {
                "type": "string",
                "description": "Command to spawn the MCP server process (e.g. 'npx -y @modelcontextprotocol/server-filesystem /path'). Required for list_tools and list_resources."
            },
            "tool_name": {
                "type": "string",
                "description": "Name of the MCP tool to call (required for call_tool)"
            },
            "arguments": {
                "type": "object",
                "description": "Arguments to pass to the MCP tool (for call_tool)"
            },
            "resource_uri": {
                "type": "string",
                "description": "URI of the resource to read (for read_resource)"
            }
        },
        "required": ["action", "command"]
    })
}
