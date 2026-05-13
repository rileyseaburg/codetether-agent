//! Successful session-recall response formatting.

use crate::rlm::RlmResult;
use crate::session::index_produce::summary_text::strip_stats_header;
use crate::tool::ToolResult;

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
