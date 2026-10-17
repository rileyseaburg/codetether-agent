//! Shared client ownership and construction of wrappers from discovery results.
use crate::{Client, McpToolWrapper};
use std::sync::Arc;
/// A shared MCP connection that produces local tool wrappers.
///
/// # Examples
/// ```rust,no_run
/// # async fn example<C:codetether_mcp_tools::Client>(manager:codetether_mcp_tools::McpToolManager<C>) {
/// let _tools=manager.wrappers().await;
/// # }
/// ```
pub struct McpToolManager<C> {
    pub(crate) client: Arc<C>,
}

impl<C: Client> McpToolManager<C> {
    /// Build local wrappers for every tool currently advertised by the MCP server.
    ///
    /// # Examples
    /// ```rust,no_run
    /// # async fn example<C:codetether_mcp_tools::Client>(manager:codetether_mcp_tools::McpToolManager<C>) {
    /// let _wrappers = manager.wrappers().await;
    /// # }
    /// ```
    pub async fn wrappers(&self) -> Vec<McpToolWrapper<C>> {
        self.client
            .tools()
            .await
            .into_iter()
            .map(|tool| McpToolWrapper::new(Arc::clone(&self.client), tool))
            .collect()
    }

    /// Return the underlying MCP client.
    pub fn client(&self) -> Arc<C> {
        Arc::clone(&self.client)
    }
}
