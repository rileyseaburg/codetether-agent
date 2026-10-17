//! Send A2A messages using JSON-RPC.
use super::*;
impl A2AClient {
    /// Send a message to the agent
    pub async fn send_message(&self, params: MessageSendParams) -> Result<SendMessageResponse> {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: serde_json::json!(1),
            method: "message/send".to_string(),
            params: serde_json::to_value(&params)?,
        };

        let response = self.call_rpc(request).await?;

        if let Some(error) = response.error {
            anyhow::bail!("RPC error {}: {}", error.code, error.message);
        }

        let result: SendMessageResponse = serde_json::from_value(
            response
                .result
                .ok_or_else(|| anyhow::anyhow!("No result"))?,
        )?;
        Ok(result)
    }
}
