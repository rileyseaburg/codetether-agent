//! Native process, client, and receiver operations for shared connection setup.
use super::*;

pub(super) struct Policy<H>(PhantomData<fn() -> H>);
impl<H: Application> codetether_mcp_core::registry::ConnectionHost for Policy<H> {
    type Client = McpClient<H>;
    async fn authorize(command: &str, args: &[&str]) -> Result<()> {
        H::authorize(command, args, None).await
    }
    async fn create(name: &str, command: &str, args: &[&str]) -> Result<Arc<McpClient<H>>> {
        let transport = Arc::new(ProcessTransport::spawn(command, args).await?);
        Ok(Arc::new(McpClient::with_registry(
            transport,
            Arc::new(McpRegistry::new()),
            Some(name.to_string()),
        )))
    }
    fn start_receiver(client: Arc<McpClient<H>>) {
        tokio::spawn(async move {
            client.receive_loop().await;
        });
    }
    async fn initialize(client: &McpClient<H>) -> Result<InitializeResult> {
        client.initialize().await
    }
}
