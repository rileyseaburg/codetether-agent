//! Read remote resource metadata and content.
use super::*;
impl RemoteClient {
    /// List remote resources.
    ///
    /// # Arguments
    /// Borrows the client.
    ///
    /// # Returns
    /// The remote resource descriptions.
    ///
    /// # Errors
    /// Transport, wire or decoding errors.
    ///
    /// # Examples
    /// ```rust,no_run
    /// async fn example(client: &codetether_mcp_transport::client::RemoteClient) -> anyhow::Result<()> {
    /// let _ = client.list_resources().await?;
    /// Ok(())
    /// }
    /// ```
    pub async fn list_resources(&self) -> Result<Vec<McpResource>> {
        self.connection.list_resources().await
    }

    /// Read one remote resource.
    ///
    /// # Arguments
    /// uri identifies the remote resource.
    ///
    /// # Returns
    /// The remote resource content.
    ///
    /// # Errors
    /// Transport, wire or decoding errors.
    ///
    /// # Examples
    /// ```rust,no_run
    /// async fn example(client: &codetether_mcp_transport::client::RemoteClient) -> anyhow::Result<()> {
    /// let _ = client.read_resource("example://resource").await?;
    /// Ok(())
    /// }
    /// ```
    pub async fn read_resource(&self, uri: &str) -> Result<ReadResourceResult> {
        self.connection.read_resource(uri).await
    }
}
