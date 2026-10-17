//! list tools action with per-call connection lifecycle.
use crate::{Host, Resources};
use anyhow::Result;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::{Value, json};
pub(super) async fn run<H: Host>(
    cmd: &str,
    cmd_args: &[&str],
    approval_id: Option<&str>,
) -> Result<ToolResult> {
    let manager = super::connect::manager::<H>(cmd, &cmd_args, approval_id).await?;
    let wrappers = manager.wrappers().await;
    let result: Vec<Value> = wrappers
        .iter()
        .map(|t| {
            json!({
                "name": t.name(),
                "description": t.description(),
                "input_schema": t.parameters(),
            })
        })
        .collect();
    manager.client().close().await?;
    Ok(ToolResult::success(serde_json::to_string_pretty(&result)?))
}
