//! Deliver a received response to the matching request channel.
use super::RpcConnection;
use codetether_mcp_core::types::JsonRpcResponse;
impl RpcConnection {
    /// Remove the pending channel and deliver its response, or log an unknown ID.
    pub async fn deliver(&self, response: JsonRpcResponse) {
        if let Some(tx) = self.pending_requests.write().await.remove(&response.id) {
            let _ = tx.send(response);
        } else {
            tracing::warn!(target: "codetether_agent::mcp::client", "Received response for unknown request: {:?}", response.id);
        }
    }
}
