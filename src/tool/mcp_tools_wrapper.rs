//! Endpoint-bound construction of remote MCP tool wrappers.

use super::McpToolWrapper;
use std::sync::Arc;

impl McpToolWrapper {
    pub fn new(client: Arc<crate::mcp::McpClient>, tool: crate::mcp::McpTool) -> Self {
        Self::new_scoped(client, tool, &uuid::Uuid::new_v4().to_string())
    }

    pub(super) fn new_scoped(
        client: Arc<crate::mcp::McpClient>,
        tool: crate::mcp::McpTool,
        authority: &str,
    ) -> Self {
        let id = format!("mcp:{authority}:{}", tool.name);
        Self { client, tool, id }
    }
}
