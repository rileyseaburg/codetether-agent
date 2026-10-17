//! Remote tool listing and invocation after caller authorization.
use crate::rpc::RpcConnection;
use anyhow::Result;
use codetether_mcp_core::types::*;
use serde_json::Value;
impl RpcConnection {
    /// Request the remote tool catalog.
    ///
    /// # Arguments
    /// Parameters identify the remote method input; the connection owns transport state.
    /// # Returns
    /// The caller decides how to cache the returned tools.
    /// # Errors
    /// Returns request/transport errors or invalid response decoding errors.
    pub async fn list_tools(&self) -> Result<Vec<McpTool>> {
        let response = self.request("tools/list", None).await?;
        let result: ListToolsResult = serde_json::from_value(response)?;
        Ok(result.tools)
    }
    /// Call a remote tool using already-authorized arguments.
    ///
    /// # Arguments
    /// Parameters identify the remote method input; the connection owns transport state.
    /// # Returns
    /// The caller applies permission checks and removes local approval metadata first.
    /// # Errors
    /// Returns request/transport errors or invalid response decoding errors.
    pub async fn call_tool(&self, name: &str, arguments: Value) -> Result<CallToolResult> {
        let params = CallToolParams {
            name: name.to_string(),
            arguments,
        };
        let response = self
            .request("tools/call", Some(serde_json::to_value(&params)?))
            .await?;
        Ok(serde_json::from_value(response)?)
    }
}
