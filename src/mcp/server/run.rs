//! Transport receive loop and response delivery.
use super::*;
impl McpServer {
    /// Run the MCP server (main loop)
    pub async fn run(&self) -> Result<()> {
        tracing::info!(target: "codetether_agent::mcp::server", "Starting MCP server...");

        // Register tools before starting
        self.setup_tools().await;

        loop {
            match self.transport.receive().await? {
                Some(McpMessage::Request(request)) => {
                    let response = self.handle_request(request).await;
                    self.transport.send_response(response).await?;
                }
                Some(McpMessage::Notification(notification)) => {
                    self.handle_notification(notification).await;
                }
                Some(McpMessage::Response(response)) => {
                    // We received a response (shouldn't happen in server mode)
                    tracing::warn!(target: "codetether_agent::mcp::server", "Unexpected response received: {:?}", response.id);
                }
                None => {
                    tracing::info!(target: "codetether_agent::mcp::server", "Transport closed, shutting down MCP server");
                    break;
                }
            }
        }

        Ok(())
    }
}
