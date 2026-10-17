//! call tool action with per-call connection lifecycle.
use crate::{Host, Resources};
use anyhow::Result;
use codetether_mcp_tools::Client;
use codetether_tool_core::ToolResult;
use serde_json::Value;
pub(super) async fn run<H: Host>(
    cmd: &str,
    cmd_args: &[&str],
    approval_id: Option<&str>,
    args: &Value,
) -> Result<ToolResult> {
    let tool_name = args["tool_name"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Missing 'tool_name' for call_tool"))?;
    let arguments = super::args::with_approval(args["arguments"].clone(), approval_id);

    let manager = super::connect::manager::<H>(cmd, &cmd_args, approval_id).await?;
    let client = manager.client();
    let result = client.call_tool(tool_name, arguments).await?;
    client.close().await?;

    Ok(codetether_mcp_tools::convert::result(result))
}
