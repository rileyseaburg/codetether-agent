//! Response formatting for `context_summarize`.

use codetether_context_policy::summary_index::types::SummaryNode;
use codetether_tool_core::ToolResult;

/// Format a cached summary response.
pub fn cached(node: &SummaryNode) -> ToolResult {
    ToolResult::success(format!(
        "Cached summary (tokens≈{}):\n{}",
        node.target_tokens, node.content
    ))
}

/// Format a newly produced summary response.
pub fn produced(node: &SummaryNode) -> ToolResult {
    ToolResult::success(format!(
        "Produced summary (tokens≈{}):\n{}",
        node.target_tokens, node.content
    ))
}
