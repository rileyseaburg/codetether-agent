//! Drive incoming remote protocol messages.
use super::*;
impl RemoteClient {
    /// Message receive loop
    ///
    /// # Arguments
    /// Borrows the client. Drive this concurrently with outgoing requests.
    ///
    /// # Returns
    /// Completes when the transport closes or receiving fails.
    ///
    /// # Examples
    /// ```rust,no_run
    /// async fn example(client: &codetether_mcp_transport::client::RemoteClient) -> anyhow::Result<()> {
    /// let _ = client.receive_loop().await;
    /// Ok(())
    /// }
    /// ```
    pub async fn receive_loop(&self) {
        loop {
            match self.connection.transport().receive().await {
                Ok(Some(message)) => {
                    self.handle_message(message).await;
                }
                Ok(None) => {
                    tracing::info!(target: "codetether_agent::mcp::client", "MCP connection closed");
                    break;
                }
                Err(e) => {
                    tracing::error!(target: "codetether_agent::mcp::client", "Error receiving MCP message: {}", e);
                    break;
                }
            }
        }
    }
}
