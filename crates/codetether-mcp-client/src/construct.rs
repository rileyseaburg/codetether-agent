//! Application MCP client construct adapter.
use super::*;
impl<H: Application> McpClient<H> {
    /// Connect to an MCP server via subprocess
    pub async fn connect_subprocess(command: &str, args: &[&str]) -> Result<Arc<Self>> {
        Self::connect_subprocess_with_approval(command, args, None).await
    }

    /// Create a new MCP client with custom transport
    pub fn new(transport: Arc<dyn Transport>) -> Self {
        Self {
            _application: PhantomData,
            inner: codetether_mcp_transport::client::RemoteClient::new(transport, None),
            registry: Arc::new(McpRegistry::new()),
        }
    }

    /// Create a new MCP client with a shared registry for multi-server management
    pub fn with_registry(
        transport: Arc<dyn Transport>,
        registry: Arc<McpRegistry<H>>,
        name: Option<String>,
    ) -> Self {
        Self {
            _application: PhantomData,
            inner: codetether_mcp_transport::client::RemoteClient::new(transport, name),
            registry,
        }
    }
}
