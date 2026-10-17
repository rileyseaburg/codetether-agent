//! list resources action with per-call connection lifecycle.
use crate::{Host, Resources};
use anyhow::Result;
use codetether_tool_core::ToolResult;
use serde_json::{Value, json};
pub(super) async fn run<H: Host>(
    cmd: &str,
    cmd_args: &[&str],
    approval_id: Option<&str>,
) -> Result<ToolResult> {
    let manager = super::connect::manager::<H>(cmd, &cmd_args, approval_id).await?;
    let client = manager.client();
    let resources = client.list_resources().await?;
    let result: Vec<Value> = resources
        .iter()
        .map(|r| {
            json!({
                "uri": r.uri,
                "name": r.name,
                "description": r.description,
                "mime_type": r.mime_type,
            })
        })
        .collect();
    client.close().await?;
    Ok(ToolResult::success(serde_json::to_string_pretty(&result)?))
}
