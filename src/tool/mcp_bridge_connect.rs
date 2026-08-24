//! MCP bridge connection helper.

use super::super::mcp_tools::McpToolManager;
use anyhow::Result;

pub(super) async fn manager(
    cmd: &str,
    cmd_args: &[&str],
    allow_network: bool,
) -> Result<McpToolManager> {
    McpToolManager::connect_subprocess_authorized(cmd, cmd_args, allow_network).await
}