//! Successful session-recall response formatting.

use codetether_context_policy::summary_index::text::strip_stats_header;
use codetether_rlm::RlmResult;
use codetether_tool_core::ToolResult;

pub(super) fn format(sources: &[String], result: &RlmResult) -> ToolResult {
    let clean = strip_stats_header(&result.processed);
    ToolResult::success(format!(
        "Recalled from {} session(s): {}\n(RLM: {} → {} tokens, {} iterations)\n\n{clean}",
        sources.len(),
        sources.join(", "),
        result.stats.input_tokens,
        result.stats.output_tokens,
        result.stats.iterations,
    ))
}
