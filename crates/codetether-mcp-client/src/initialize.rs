//! Application MCP client initialize adapter.
use super::*;
impl<H: Application> McpClient<H> {
    /// Initialize the connection with the server
    pub async fn initialize(&self) -> Result<InitializeResult> {
        self.inner.initialize(H::version()).await
    }

    /// Refresh the list of available tools
    pub async fn refresh_tools(&self) -> Result<Vec<McpTool>> {
        self.inner.refresh_tools().await
    }

    /// Message receive loop
    pub(super) async fn receive_loop(&self) {
        self.inner.receive_loop().await
    }
}
