//! MCP bridge connection helper.

use super::super::mcp_tools::McpToolManager;
use anyhow::Result;

pub(super) async fn manager(
    cmd: &str,
    cmd_args: &[&str],
    approval_id: Option<&str>,
) -> Result<McpToolManager> {
    McpToolManager::connect_subprocess_with_approval(cmd, cmd_args, approval_id).await
}
