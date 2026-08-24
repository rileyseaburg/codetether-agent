//! Discovery and client access for an MCP tool manager.

use super::{McpToolManager, McpToolWrapper};
use std::sync::Arc;

impl McpToolManager {
    /// Build local wrappers for every tool currently advertised by the MCP server.
    pub async fn wrappers(&self) -> Vec<McpToolWrapper> {
        self.client
            .tools()
            .await
            .into_iter()
            .map(|tool| McpToolWrapper::new_scoped(Arc::clone(&self.client), tool, &self.authority))
            .collect()
    }

    /// Return the underlying MCP client.
    pub fn client(&self) -> Arc<crate::mcp::McpClient> {
        Arc::clone(&self.client)
    }
}
