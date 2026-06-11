//! Approval-aware MCP subprocess connection helpers.

use super::{McpClient, ProcessTransport};
use anyhow::Result;
use std::sync::Arc;

impl McpClient {
    pub async fn connect_subprocess_with_approval(
        command: &str,
        args: &[&str],
        approval_id: Option<&str>,
    ) -> Result<Arc<Self>> {
        super::super::subprocess_policy::guard(command, args, approval_id).await?;
        let transport = Arc::new(ProcessTransport::spawn(command, args).await?);
        let client = Arc::new(Self::new(transport));
        let client_clone = Arc::clone(&client);
        tokio::spawn(async move {
            client_clone.receive_loop().await;
        });
        client.initialize().await?;
        Ok(client)
    }
}
