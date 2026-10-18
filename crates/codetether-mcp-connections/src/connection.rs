//! Connection identity and a public discovery summary.
use std::sync::Arc;
/// Name, launch command and current tool count for a connected MCP server.
/// # Examples
/// ```
/// let server = codetether_mcp_connections::ServerSummary {
///     name: "files".into(), command: "files-server".into(), tool_count: 0,
/// };
/// assert_eq!(server.tool_count, 0);
/// ```
#[derive(Clone, Debug)]
pub struct ServerSummary {
    /// User-assigned server name.
    pub name: String,
    /// Original subprocess command text.
    pub command: String,
    /// Number of tools currently advertised by the client.
    pub tool_count: usize,
}
pub(crate) struct Connection<C> {
    pub(crate) name: String,
    pub(crate) command: String,
    pub(crate) client: Arc<C>,
}
impl<C> Clone for Connection<C> {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            command: self.command.clone(),
            client: Arc::clone(&self.client),
        }
    }
}
