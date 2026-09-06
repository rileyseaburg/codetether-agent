//! MCP tool wrappers for exposing external MCP tools through the local Tool trait.

use std::sync::Arc;

#[path = "mcp_tools_connect.rs"]
mod connect;
#[path = "mcp_tools_convert.rs"]
pub(super) mod convert;
#[path = "mcp_tools_wrapper.rs"]
mod wrapper;
pub use wrapper::McpToolWrapper;

/// Manages a connection to an MCP server and produces local tool wrappers.
pub struct McpToolManager {
    client: Arc<crate::mcp::McpClient>,
}

impl McpToolManager {
    /// Build local wrappers for every tool currently advertised by the MCP server.
    pub async fn wrappers(&self) -> Vec<McpToolWrapper> {
        self.client
            .tools()
            .await
            .into_iter()
            .map(|tool| McpToolWrapper::new(Arc::clone(&self.client), tool))
            .collect()
    }

    /// Return the underlying MCP client.
    pub fn client(&self) -> Arc<crate::mcp::McpClient> {
        Arc::clone(&self.client)
    }
}
