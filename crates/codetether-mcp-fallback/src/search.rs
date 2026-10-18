//! Native-search execution and MCP text-result conversion.

use anyhow::Result;
use codetether_mcp_core::types::{CallToolResult, ToolContent};
use serde_json::Value;

pub(super) fn search_files(args: Value) -> Result<CallToolResult> {
    text(codetether_search::native::search_files(&args)?, false)
}

pub(super) fn grep_search(args: Value) -> Result<CallToolResult> {
    text(codetether_search::native::grep(&args)?, false)
}

fn text(output: String, is_error: bool) -> Result<CallToolResult> {
    Ok(CallToolResult {
        content: vec![ToolContent::Text { text: output }],
        is_error,
    })
}
