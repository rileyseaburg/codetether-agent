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
        Ok(Self {
            client,
            authority: authority(command, args),
        })
    }

    pub(crate) async fn connect_subprocess_authorized(
        command: &str,
        args: &[&str],
        allow_network: bool,
    ) -> Result<Self> {
        let client =
            crate::mcp::McpClient::connect_subprocess_authorized(command, args, allow_network)
                .await?;
        Ok(Self {
            client,
            authority: authority(command, args),
        })
    }
}

fn authority(command: &str, args: &[&str]) -> String {
    let invocation = serde_json::json!({"command": command, "argv": args});
    crate::runtime_policy::invocation_scope::for_tool("mcp", &invocation).resource
}
