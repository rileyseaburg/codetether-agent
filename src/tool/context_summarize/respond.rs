//! Response formatting for `context_summarize`.

use super::super::ToolResult;
use crate::session::index::types::SummaryNode;

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
