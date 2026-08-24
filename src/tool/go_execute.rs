//! Validation, network authorization, and dispatch for the Go tool.

use super::{GoParams, GoTool};
use crate::tool::ToolResult;
use anyhow::{Context, Result};
use serde_json::{Value, json};

pub(super) async fn run(tool: &GoTool, params: Value) -> Result<ToolResult> {
    let parsed: GoParams = serde_json::from_value(params.clone()).context("Invalid params")?;
    crate::tool::network_access::guard!("go", &params);
    match parsed.action.as_str() {
        "execute" => tool.execute_go(parsed).await,
        "watch" => tool.watch_go(parsed).await,
        "status" => tool.check_status(parsed).await,
        _ => Ok(ToolResult::structured_error(
            "INVALID_ACTION",
            "go",
            &format!(
                "Unknown action: '{}'. Valid actions: execute, watch, status",
                parsed.action
            ),
            None,
            Some(json!({
                "action": "execute",
                "task": "implement feature X with tests"
            })),
        )),
    }
}
