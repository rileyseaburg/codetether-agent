//! MCP Bridge Tool: Connect to and invoke tools from external MCP servers
//!
//! This tool enables agents (including the A2A worker) to connect to external
//! MCP (Model Context Protocol) servers and invoke their tools.

use super::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{Value, json};

/// MCP Bridge Tool - Connect to external MCP servers and call their tools
pub struct McpBridgeTool;

impl Default for McpBridgeTool {
    fn default() -> Self {
        Self::new()
    }
}

impl McpBridgeTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for McpBridgeTool {
    fn id(&self) -> &str {
        "mcp"
    }

    fn name(&self) -> &str {
        "MCP Bridge"
    }

    fn description(&self) -> &str {
        "Connect to an MCP (Model Context Protocol) server and invoke its tools. \
         Actions: 'list_tools' to discover available tools from an MCP server, \
         'call_tool' to invoke a specific tool, 'list_resources' to list available resources, \
         'read_resource' to read a resource by URI."
    }

    fn parameters(&self) -> Value {
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

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let action = args["action"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing 'action' parameter"))?;
        let command = args["command"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing 'command' parameter"))?;

        // Parse command into parts
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return Ok(ToolResult::error("Empty command"));
        }

        let cmd = parts[0];
        let cmd_args: Vec<&str> = parts[1..].to_vec();

        match action {
            "list_tools" => {
                let client =
                    crate::mcp::McpClient::connect_subprocess(cmd, &cmd_args).await?;
                let tools = client.tools().await;
                let result: Vec<Value> = tools
                    .iter()
                    .map(|t| {
                        json!({
                            "name": t.name,
                            "description": t.description,
                            "input_schema": t.input_schema,
                        })
                    })
                    .collect();
                client.close().await?;
                Ok(ToolResult::success(serde_json::to_string_pretty(&result)?))
            }
            "call_tool" => {
                let tool_name = args["tool_name"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("Missing 'tool_name' for call_tool"))?;
                let arguments = args["arguments"].clone();
                let arguments = if arguments.is_null() {
                    json!({})
                } else {
                    arguments
                };

                let client =
                    crate::mcp::McpClient::connect_subprocess(cmd, &cmd_args).await?;
                let result = client.call_tool(tool_name, arguments).await?;
                client.close().await?;

                let output: String = result
                    .content
                    .iter()
                    .map(|c| match c {
                        crate::mcp::ToolContent::Text { text } => text.clone(),
                        crate::mcp::ToolContent::Image { data, mime_type } => {
                            format!("[image: {} ({} bytes)]", mime_type, data.len())
                        }
                        crate::mcp::ToolContent::Resource { resource } => {
                            serde_json::to_string(resource).unwrap_or_default()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                if result.is_error {
                    Ok(ToolResult::error(output))
                } else {
                    Ok(ToolResult::success(output))
                }
            }
            "list_resources" => {
                let client =
                    crate::mcp::McpClient::connect_subprocess(cmd, &cmd_args).await?;
                let resources = client.list_resources().await?;
                let result: Vec<Value> = resources
                    .iter()
                    .map(|r| {
                        json!({
                            "uri": r.uri,
                            "name": r.name,
                            "description": r.description,
                            "mime_type": r.mime_type,
                        })
                    })
                    .collect();
                client.close().await?;
                Ok(ToolResult::success(serde_json::to_string_pretty(&result)?))
            }
            "read_resource" => {
                let uri = args["resource_uri"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("Missing 'resource_uri' for read_resource"))?;

                let client =
                    crate::mcp::McpClient::connect_subprocess(cmd, &cmd_args).await?;
                let result = client.read_resource(uri).await?;
                client.close().await?;
                Ok(ToolResult::success(serde_json::to_string_pretty(&result)?))
            }
            _ => Ok(ToolResult::error(format!("Unknown action: {}", action))),
        }
    }
}
