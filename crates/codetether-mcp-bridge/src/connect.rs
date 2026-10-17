//! MCP bridge connection helper.

use crate::Host;
use anyhow::Result;
use codetether_mcp_tools::McpToolManager;

pub(super) async fn manager<H: Host>(
    cmd: &str,
    cmd_args: &[&str],
    approval_id: Option<&str>,
) -> Result<McpToolManager<H::Client>> {
    McpToolManager::connect_subprocess_with_approval(cmd, cmd_args, approval_id).await
}
