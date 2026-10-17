//! read resource action with per-call connection lifecycle.
use crate::{Host, Resources};
use anyhow::Result;
use codetether_tool_core::ToolResult;
use serde_json::Value;
pub(super) async fn run<H: Host>(
    cmd: &str,
    cmd_args: &[&str],
    approval_id: Option<&str>,
    args: &Value,
) -> Result<ToolResult> {
    let uri = args["resource_uri"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Missing 'resource_uri' for read_resource"))?;

    let manager = super::connect::manager::<H>(cmd, &cmd_args, approval_id).await?;
    let client = manager.client();
    let result = client.read_resource(uri).await?;
    client.close().await?;
    Ok(ToolResult::success(serde_json::to_string_pretty(&result)?))
}
