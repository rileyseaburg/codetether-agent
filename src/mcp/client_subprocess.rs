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
        Self::connect_subprocess_with_runtime(command, args, approval_id, false, "mcp-api").await
    }

    pub(crate) async fn connect_subprocess_with_runtime(
        command: &str,
        args: &[&str],
        approval_id: Option<&str>,
        network_allowed: bool,
        session_id: &str,
    ) -> Result<Arc<Self>> {
        super::super::subprocess_policy::guard_scoped(
            command,
            args,
            approval_id,
            network_allowed,
            session_id,
        )
        .await?;
        Self::connect_subprocess_authorized(command, args, network_allowed).await
    }

    pub(crate) async fn connect_subprocess_authorized(
        command: &str,
        args: &[&str],
        allow_network: bool,
    ) -> Result<Arc<Self>> {
        let transport = Arc::new(ProcessTransport::spawn(command, args, allow_network).await?);
        let client = Arc::new(Self::new(transport));
        let client_clone = Arc::clone(&client);
        tokio::spawn(async move {
            client_clone.receive_loop().await;
        });
        client.initialize().await?;
        Ok(client)
    }
}
