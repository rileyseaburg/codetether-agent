//! Send correlated requests and await their existing timeout/error policy.
use super::RpcConnection;
use anyhow::Result;
use codetether_mcp_core::types::{JsonRpcRequest, RequestId};
use serde_json::Value;
use std::sync::atomic::Ordering;
use std::time::Duration;
use tokio::{sync::oneshot, time::timeout};
impl RpcConnection {
    /// Send a JSON-RPC request and wait for its correlated response.
    ///
    /// # Arguments
    /// * `method` — Remote JSON-RPC method.
    /// * `params` — Optional serialized arguments.
    /// # Returns
    /// The result value from the response.
    /// # Errors
    /// Returns transport, closed-channel, 30-second timeout, remote-error or empty-result errors.
    /// # Examples
    /// ```no_run
    /// # async fn example(connection: &codetether_mcp_transport::rpc::RpcConnection) -> anyhow::Result<()> {
    /// let result = connection.request("tools/list", None).await?;
    /// # Ok(()) }
    /// ```
    pub async fn request(&self, method: &str, params: Option<Value>) -> Result<Value> {
        let id = RequestId::Number(self.request_id.fetch_add(1, Ordering::SeqCst));
        let request = JsonRpcRequest::new(id.clone(), method, params);

        // Create response channel
        let (tx, rx) = oneshot::channel();
        self.pending_requests.write().await.insert(id.clone(), tx);

        // Send request
        self.transport.send_request(request).await?;

        // Wait for response with timeout
        let response = timeout(Duration::from_secs(30), rx)
            .await
            .map_err(|_| anyhow::anyhow!("Request timed out"))??;

        if let Some(error) = response.error {
            return Err(anyhow::anyhow!(
                "MCP error {}: {}",
                error.code,
                error.message
            ));
        }

        response
            .result
            .ok_or_else(|| anyhow::anyhow!("Empty response"))
    }
}
