//! Native process, client, and receiver operations for shared connection setup.
use super::*;

pub(super) struct Native;
impl codetether_mcp_core::registry::ConnectionHost for Native {
    type Client = McpClient;
    async fn authorize(command: &str, args: &[&str]) -> Result<()> {
        crate::mcp::subprocess_policy::guard(command, args, None).await
    }
    async fn create(name: &str, command: &str, args: &[&str]) -> Result<Arc<McpClient>> {
        let transport = Arc::new(ProcessTransport::spawn(command, args).await?);
        Ok(Arc::new(McpClient::with_registry(
            transport,
            Arc::new(McpRegistry::new()),
            Some(name.to_string()),
        )))
    }
    fn start_receiver(client: Arc<McpClient>) {
        tokio::spawn(async move {
            client.receive_loop().await;
        });
    }
    async fn initialize(client: &McpClient) -> Result<InitializeResult> {
        client.initialize().await
    }
}
