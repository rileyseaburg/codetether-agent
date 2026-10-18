//! Connection-registry binding for the policy-bound client.
use super::{Application, McpClient};
use anyhow::Result;
use std::sync::Arc;
impl<H: Application> codetether_mcp_connections::ConnectionClient for McpClient<H> {
    async fn connect_subprocess(command: &str, args: &[&str]) -> Result<Arc<Self>> {
        McpClient::connect_subprocess(command, args).await
    }
}
