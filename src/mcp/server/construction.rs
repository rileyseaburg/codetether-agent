//! Server construction and inert default-registration hook.
use super::*;
impl McpServer {
    /// Create a new MCP server over stdio
    pub fn new_stdio() -> Self {
        // Use Arc's unsized coercion to convert Arc<StdioTransport> -> Arc<dyn Transport>
        let transport: Arc<dyn Transport> = Arc::new(StdioTransport::new());
        Self::new(transport)
    }

    /// Create a new MCP server for in-process/local usage.
    ///
    /// Unlike [`Self::new_stdio`], this does not spawn any stdio reader/writer threads
    /// and will not lock stdout. This is intended for CLI flows that need to query
    /// tool metadata or invoke tools directly without running a long-lived stdio server.
    pub fn new_local() -> Self {
        let transport: Arc<dyn Transport> = Arc::new(super::super::transport::NullTransport::new());
        Self::new(transport)
    }

    /// Create a new MCP server with custom transport
    pub fn new(transport: Arc<dyn Transport>) -> Self {
        let mut server = Self {
            inner: codetether_mcp_server::ServerState::new(transport, env!("CARGO_PKG_VERSION")),
        };

        // Register default tools
        server.register_default_tools();

        server
    }

    /// Register default CodeTether tools
    pub(super) fn register_default_tools(&mut self) {
        // These will be registered synchronously in the constructor
        // The actual tool handlers will be added in run()
    }
}
