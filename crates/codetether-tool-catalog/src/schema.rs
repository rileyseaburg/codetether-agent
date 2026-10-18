//! Schema for the virtual tool catalog lookup operation.
use codetether_provider_core::ToolDefinition;
use serde_json::json;

/// Describe the virtual tool that lists tools or returns one complete schema.
/// # Arguments
/// This function takes no arguments.
/// # Returns
/// A `list_tools` schema with optional `query` and `tool` string parameters.
/// # Examples
/// ```
/// let schema = codetether_tool_catalog::list_tools_bootstrap_definition();
/// assert_eq!(schema.name, "list_tools");
/// ```
pub fn list_tools_bootstrap_definition() -> ToolDefinition {
    ToolDefinition {
        name: "list_tools".to_string(),
        description: "List available tools (optionally filter by query) or fetch a full schema for one tool via {\"tool\":\"name\"}.".to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "Optional substring filter over tool names/descriptions"
                },
                "tool": {
                    "type": "string",
                    "description": "Optional exact tool name to return full schema"
                }
            }
        }),
    }
}
