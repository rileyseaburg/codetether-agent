//! Bridge metadata and policy-bound action execution.
use crate::{Host, McpBridgeTool};
use anyhow::Result;
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::Value;
#[async_trait]
impl<H: Host> Tool for McpBridgeTool<H> {
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
        super::schema::parameters()
    }
    async fn execute(&self, args: Value) -> Result<ToolResult> {
        super::execute::run::<H>(args).await
    }
}
