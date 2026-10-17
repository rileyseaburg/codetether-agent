//! Read and update the registry-facing server name.
use super::*;
impl RemoteClient {
    /// Get the server name if set
    ///
    /// # Arguments
    /// Borrows the client.
    ///
    /// # Returns
    /// The optional registry-facing name.
    ///
    /// # Examples
    /// ```rust,no_run
    /// async fn example(client: &codetether_mcp_transport::client::RemoteClient) -> anyhow::Result<()> {
    /// let _ = client.server_name().await;
    /// Ok(())
    /// }
    /// ```
    pub async fn server_name(&self) -> Option<String> {
        self.server_name.read().await.clone()
    }

    /// Set the server name for registry tracking
    ///
    /// # Arguments
    /// name replaces the registry-facing name.
    ///
    /// # Returns
    /// Completes after updating the name.
    ///
    /// # Examples
    /// ```rust,no_run
    /// async fn example(client: &codetether_mcp_transport::client::RemoteClient) -> anyhow::Result<()> {
    /// let _ = client.set_server_name("example".into()).await;
    /// Ok(())
    /// }
    /// ```
    pub async fn set_server_name(&self, name: String) {
        *self.server_name.write().await = Some(name);
    }
}
