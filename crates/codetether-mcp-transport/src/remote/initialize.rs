//! MCP initialization exchange with explicit application identity.
use crate::rpc::RpcConnection;
use anyhow::Result;
use codetether_mcp_core::types::*;
impl RpcConnection {
    /// Exchange initialization with the supplied application version.
    ///
    /// # Arguments
    /// Parameters identify the remote method input; the connection owns transport state.
    /// # Returns
    /// The caller stores metadata and sends the initialized notification afterward.
    /// # Errors
    /// Returns request/transport errors or invalid response decoding errors.
    pub async fn initialize(&self, client_version: &str) -> Result<InitializeResult> {
        let params = InitializeParams {
            protocol_version: PROTOCOL_VERSION.to_string(),
            capabilities: ClientCapabilities {
                roots: Some(RootsCapability { list_changed: true }),
                sampling: Some(SamplingCapability {}),
                experimental: None,
            },
            client_info: ClientInfo {
                name: "codetether".to_string(),
                version: client_version.to_string(),
            },
        };

        let response = self
            .request("initialize", Some(serde_json::to_value(&params)?))
            .await?;
        let result: InitializeResult = serde_json::from_value(response)?;

        Ok(result)
    }
}
