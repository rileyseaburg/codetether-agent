//! Sandboxed fallback implementation for MCP run_command.

#[path = "fallback_run_command_args.rs"]
mod args;

use super::types::{CallToolResult, ToolContent};
use crate::tool::{Tool, ToolResult, bash::BashTool};
use anyhow::Result;
use serde_json::Value;

pub(super) fn call(args: Value) -> Result<CallToolResult> {
    if let Some(blocked) = super::tool_policy::blocked("run_command", &args) {
        return Ok(blocked);
    }
    let bash_args = args::translate(&args)?;
    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current()
            .block_on(async { BashTool::new().execute(bash_args).await })
    });
    Ok(from_tool_result(result.unwrap_or_else(|error| {
        ToolResult::error(error.to_string())
    })))
}

fn from_tool_result(result: ToolResult) -> CallToolResult {
    CallToolResult {
        content: vec![ToolContent::Text {
            text: result.output,
        }],
        is_error: !result.success,
    }
}

#[cfg(test)]
#[path = "fallback_run_command_tests.rs"]
mod tests;
