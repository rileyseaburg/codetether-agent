//! In-process transport that never acquires standard I/O.
use crate::*;

/// Null transport for local/in-process MCP usage.
///
/// This transport intentionally does **not** spawn any stdio reader/writer threads
/// and does not lock stdout. It is suitable for CLI commands that want to reuse the
/// MCP tool registry (e.g. `codetether mcp list-tools` and `codetether mcp call`)
/// without running a stdio server.
#[derive(Debug, Default, Clone)]
pub struct NullTransport;

impl NullTransport {
    /// Construct an inert transport without spawning tasks or touching stdio.
    ///
    /// # Examples
    /// ```
    /// let transport = codetether_mcp_transport::NullTransport::new();
    /// ```
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Transport for NullTransport {
    async fn send_request(&self, _request: JsonRpcRequest) -> Result<()> {
        Ok(())
    }

    async fn send_response(&self, _response: JsonRpcResponse) -> Result<()> {
        Ok(())
    }

    async fn send_notification(&self, _notification: JsonRpcNotification) -> Result<()> {
        Ok(())
    }

    async fn receive(&self) -> Result<Option<McpMessage>> {
        Ok(None)
    }

    async fn close(&self) -> Result<()> {
        Ok(())
    }
}
