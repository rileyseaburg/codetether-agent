//! Remote MCP prompts operations.
use crate::rpc::RpcConnection;
use anyhow::Result;
use codetether_mcp_core::types::*;
use serde_json::Value;
impl RpcConnection {
    /// List available prompts
    /// Request the remote prompt catalog.
    ///
    /// # Arguments
    /// Parameters identify the remote method input; the connection owns transport state.
    /// # Returns
    /// Returns the decoded prompt records.
    /// # Errors
    /// Returns request/transport errors or invalid response decoding errors.
    pub async fn list_prompts(&self) -> Result<Vec<McpPrompt>> {
        let response = self.request("prompts/list", None).await?;
        let result: ListPromptsResult = serde_json::from_value(response)?;
        Ok(result.prompts)
    }

    /// Get a prompt
    /// Request a named prompt with supplied arguments.
    ///
    /// # Arguments
    /// Parameters identify the remote method input; the connection owns transport state.
    /// # Returns
    /// Returns the decoded prompt result.
    /// # Errors
    /// Returns request/transport errors or invalid response decoding errors.
    pub async fn get_prompt(&self, name: &str, arguments: Value) -> Result<GetPromptResult> {
        let params = GetPromptParams {
            name: name.to_string(),
            arguments,
        };
        let response = self
            .request("prompts/get", Some(serde_json::to_value(&params)?))
            .await?;
        let result: GetPromptResult = serde_json::from_value(response)?;
        Ok(result)
    }
}
