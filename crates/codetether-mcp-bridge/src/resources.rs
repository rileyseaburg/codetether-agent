//! Additional MCP resource operations required by the bridge.
use anyhow::Result;
use async_trait::async_trait;
use codetether_mcp_core::types::{McpResource, ReadResourceResult};
/// Resource and lifecycle capabilities of an authorized MCP client.
///
/// # Implementors
/// Extend the existing tool-capable client by forwarding its resource and close methods.
#[async_trait]
pub trait Resources: codetether_mcp_tools::Client {
    /// List resource descriptors advertised by the server.
    /// # Errors
    /// Returns transport or server errors.
    async fn list_resources(&self) -> Result<Vec<McpResource>>;
    /// Read the resource with the given URI.
    /// # Errors
    /// Returns transport or server errors.
    async fn read_resource(&self, uri: &str) -> Result<ReadResourceResult>;
    /// Close the per-action connection.
    /// # Errors
    /// Returns errors reported while closing the client.
    async fn close(&self) -> Result<()>;
}
