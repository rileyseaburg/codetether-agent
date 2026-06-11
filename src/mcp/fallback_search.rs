//! Native fallback MCP search adapters.

use super::types::{CallToolResult, ToolContent};
use anyhow::Result;
use serde_json::Value;

pub(super) fn search_files(args: Value) -> Result<CallToolResult> {
    text(super::fallback_search_files::run(&args)?, false)
}

pub(super) fn grep_search(args: Value) -> Result<CallToolResult> {
    text(super::fallback_grep::run(&args)?, false)
}

fn text(output: String, is_error: bool) -> Result<CallToolResult> {
    Ok(CallToolResult {
        content: vec![ToolContent::Text { text: output }],
        is_error,
    })
}
