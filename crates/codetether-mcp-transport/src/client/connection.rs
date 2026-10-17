//! Connection access, notification and close operations.
use super::*;
impl RemoteClient {
    pub(super) async fn notify(&self, method: &str, params: Option<Value>) -> Result<()> {
        self.connection.notify(method, params).await
    }

    /// Close the connection
    ///
    /// # Arguments
    /// Borrows the client.
    ///
    /// # Returns
    /// Completes when transport closure succeeds.
    ///
    /// # Errors
    /// Transport closure errors.
    ///
    /// # Examples
    /// ```rust,no_run
    /// async fn example(client: &codetether_mcp_transport::client::RemoteClient) -> anyhow::Result<()> {
    /// let _ = client.close().await?;
    /// Ok(())
    /// }
    /// ```
    pub async fn close(&self) -> Result<()> {
        self.connection.transport().close().await
    }
    /// Borrow the correlated connection without changing ownership.
    /// Outgoing tool authorization belongs to the caller.
    ///
    /// # Arguments
    /// Borrows the client.
    /// # Returns
    /// The underlying correlated connection.
    /// # Examples
    /// ```
    /// use codetether_mcp_transport::{client::RemoteClient, NullTransport};
    /// let client = RemoteClient::new(std::sync::Arc::new(NullTransport::new()), None);
    /// let connection = client.connection();
    /// ```
    pub fn connection(&self) -> &RpcConnection {
        &self.connection
    }
}
