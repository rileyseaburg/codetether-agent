//! Remote MCP resources operations.
use crate::rpc::RpcConnection;
use anyhow::Result;
use codetether_mcp_core::types::*;
impl RpcConnection {
    /// List available resources
    /// Request the remote resource catalog.
    ///
    /// # Arguments
    /// Parameters identify the remote method input; the connection owns transport state.
    /// # Returns
    /// Returns the decoded resource records.
    /// # Errors
    /// Returns request/transport errors or invalid response decoding errors.
    pub async fn list_resources(&self) -> Result<Vec<McpResource>> {
        let response = self.request("resources/list", None).await?;
        let result: ListResourcesResult = serde_json::from_value(response)?;
        Ok(result.resources)
    }

    /// Read a resource
    /// Read the named remote resource.
    ///
    /// # Arguments
    /// Parameters identify the remote method input; the connection owns transport state.
    /// # Returns
    /// Returns the decoded resource contents.
    /// # Errors
    /// Returns request/transport errors or invalid response decoding errors.
    pub async fn read_resource(&self, uri: &str) -> Result<ReadResourceResult> {
        let params = ReadResourceParams {
            uri: uri.to_string(),
        };
        let response = self
            .request("resources/read", Some(serde_json::to_value(&params)?))
            .await?;
        let result: ReadResourceResult = serde_json::from_value(response)?;
        Ok(result)
    }
}
