//! Fallback command approval, argument translation and response conversion.

#[path = "args.rs"]
mod args;

use crate::Host;
use anyhow::Result;
use codetether_mcp_core::types::{CallToolResult, ToolContent};
use codetether_tool_core::ToolResult;
use serde_json::Value;

pub(crate) fn call<H: Host>(args: Value) -> Result<CallToolResult> {
    if let Some(blocked) = H::blocked("run_command", &args) {
        return Ok(blocked);
    }
    let bash_args = args::translate(&args)?;
    let result = H::execute_bash(bash_args);
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
#[path = "tests.rs"]
mod tests;
