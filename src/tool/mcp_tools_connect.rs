//! Approval-aware MCP manager connection helper.

use super::McpToolManager;
use anyhow::Result;

impl McpToolManager {
    pub async fn connect_subprocess_with_approval(
        command: &str,
        args: &[&str],
        approval_id: Option<&str>,
    ) -> Result<Self> {
        let client =
            crate::mcp::McpClient::connect_subprocess_with_approval(command, args, approval_id)
                .await?;
        Ok(Self { client })
    }
}
