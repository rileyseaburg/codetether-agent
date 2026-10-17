//! Decode and dispatch supported autonomous task actions.

use crate::{GoTool, Host, params::GoParams};
use anyhow::{Context, Result};
use codetether_tool_core::ToolResult;
use serde_json::{Value, json};
pub(crate) async fn execute<H: Host>(tool: &GoTool<H>, params: Value) -> Result<ToolResult> {
    let p: GoParams = serde_json::from_value(params).context("Invalid params")?;

    match p.action.as_str() {
        "execute" => crate::launch::execute(tool, p).await,
        "watch" => codetether_go_monitor::watch(p.okr_id),
        "status" => crate::status::check(p).await,
        _ => Ok(ToolResult::structured_error(
            "INVALID_ACTION",
            "go",
            &format!(
                "Unknown action: '{}'. Valid actions: execute, watch, status",
                p.action
            ),
            None,
            Some(json!({
                "action": "execute",
                "task": "implement feature X with tests"
            })),
        )),
    }
}
