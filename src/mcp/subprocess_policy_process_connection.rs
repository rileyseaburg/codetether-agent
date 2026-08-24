//! Real MCP fixture connection helpers.

use crate::mcp::McpClient;
use std::sync::Arc;
use std::time::Duration;

pub(super) async fn approved(command: &str, request_id: &str) -> Arc<McpClient> {
    tokio::time::timeout(Duration::from_secs(3), connect(command, request_id))
        .await
        .expect("MCP initialization timeout")
        .expect("approved MCP connection")
}

pub(super) async fn replay(command: &str, request_id: &str) -> anyhow::Result<Arc<McpClient>> {
    connect(command, request_id).await
}

async fn connect(command: &str, request_id: &str) -> anyhow::Result<Arc<McpClient>> {
    McpClient::connect_subprocess_with_runtime(command, &[], Some(request_id), false, "mcp-test")
        .await
}
