//! Forward bridge resource operations to the existing MCP client.
use crate::mcp::{McpClient, McpResource, ReadResourceResult};
use anyhow::Result;
#[async_trait::async_trait]
impl codetether_mcp_bridge::Resources for McpClient {
    async fn list_resources(&self) -> Result<Vec<McpResource>> {
        McpClient::list_resources(self).await
    }
    async fn read_resource(&self, uri: &str) -> Result<ReadResourceResult> {
        McpClient::read_resource(self, uri).await
    }
    async fn close(&self) -> Result<()> {
        McpClient::close(self).await
    }
}
