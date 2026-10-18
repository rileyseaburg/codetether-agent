//! Approval-aware MCP subprocess connection helpers.

use super::{Application, McpClient, ProcessTransport};
use anyhow::Result;
use std::sync::Arc;

impl<H: Application> McpClient<H> {
    /// Authorize, launch and initialize an MCP subprocess connection.
    /// # Arguments
    /// Receives the command, its arguments and an optional existing approval ID.
    /// # Returns
    /// A shared initialized client with its receive loop running.
    /// # Errors
    /// Propagates authorization, process startup and initialization failures.
    /// # Examples
    /// ```no_run
    /// # async fn example<H: codetether_mcp_client::Application>() -> anyhow::Result<()> {
    /// let client = codetether_mcp_client::McpClient::<H>::connect_subprocess_with_approval(
    ///     "mcp-server", &[], None,
    /// ).await?;
    /// client.close().await?;
    /// # Ok(()) }
    /// ```
    pub async fn connect_subprocess_with_approval(
        command: &str,
        args: &[&str],
        approval_id: Option<&str>,
    ) -> Result<Arc<Self>> {
        H::authorize(command, args, approval_id).await?;
        let transport = Arc::new(ProcessTransport::spawn(command, args).await?);
        let client = Arc::new(Self::new(transport));
        let client_clone = Arc::clone(&client);
        tokio::spawn(async move {
            client_clone.receive_loop().await;
        });
        client.initialize().await?;
        Ok(client)
    }
}
