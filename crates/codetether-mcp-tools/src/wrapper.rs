//! Shared remote definition and local tool identity.
use crate::Client;
use codetether_mcp_core::types::McpTool;
use std::sync::Arc;
/// Wrap a remote MCP definition while retaining the authorized shared client.
///
/// # Examples
/// ```rust,no_run
/// # fn example<C:codetether_mcp_tools::Client>(client:std::sync::Arc<C>,tool:codetether_mcp_core::types::McpTool) {
/// use codetether_tool_core::Tool;
/// let wrapper=codetether_mcp_tools::McpToolWrapper::new(client,tool);
/// assert!(wrapper.id().starts_with("mcp:"));
/// # }
/// ```
pub struct McpToolWrapper<C> {
    pub(crate) client: Arc<C>,
    pub(crate) tool: McpTool,
    pub(crate) id: String,
}
impl<C: Client> McpToolWrapper<C> {
    /// Construct a wrapper using the remote definition and shared client.
    ///
    /// # Arguments
    /// * `client` - Shared policy-aware MCP client.
    /// * `tool` - Definition advertised by its server.
    /// # Returns
    /// A wrapper with a stable `mcp:<name>` identifier.
    /// # Examples
    /// ```rust,no_run
    /// # fn example<C:codetether_mcp_tools::Client>(client:std::sync::Arc<C>,tool:codetether_mcp_core::types::McpTool) {
    /// let _wrapper=codetether_mcp_tools::McpToolWrapper::new(client,tool);
    /// # }
    /// ```

    /// Construct a wrapper using the remote definition and shared client.
    pub fn new(client: Arc<C>, tool: McpTool) -> Self {
        let id = format!("mcp:{}", tool.name);
        Self { client, tool, id }
    }
}
