//! Non-converged session-recall fallback formatting.

use crate::rlm::RlmResult;
use crate::session::index_produce::summary_text::strip_stats_header;
use crate::tool::ToolResult;

pub(super) fn non_converged(
    context: &str,
    sources: &[String],
    result: &RlmResult,
) -> ToolResult {
    tracing::warn!(
        input_tokens = result.stats.input_tokens,
        output_tokens = result.stats.output_tokens,
        "RLM recall did not converge"
    );
    let fallback = fallback_text(context, &result.processed);
    ToolResult::success(format!(
        "Recalled from {} session(s) (degraded RLM):\n\n{fallback}",
        sources.len(),
    ))
}

fn fallback_text(context: &str, processed: &str) -> String {
    if processed.trim().is_empty() {
        context.chars().take(4000).collect()
    } else {
        strip_stats_header(processed).to_string()
    }
}
