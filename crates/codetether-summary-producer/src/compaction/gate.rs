use codetether_rlm::{RlmChunker, RlmResult};
use codetether_summary_quality::try_bounded_summary;
/// Accept the RLM summary if it passes [`try_bounded_summary`];
/// otherwise log and fall back to deterministic chunk compression.
///
/// # Arguments
/// * `result` - RLM result to validate.
/// * `target_tokens` - Output token budget.
/// * `fallback_input` - Source text for deterministic compression.
/// * `where_` - Static context attached to the fallback warning.
/// # Returns
/// Accepted bounded summary, or a deterministic chunk summary.
/// # Examples
/// ```text
/// let summary = gated_summary_or_chunk(result, 512, transcript, "compaction");
/// ```

pub fn gated_summary_or_chunk(
    result: RlmResult,
    target_tokens: usize,
    fallback_input: &str,
    where_: &'static str,
) -> String {
    if let Some(text) = try_bounded_summary(&result, target_tokens) {
        return text;
    }
    tracing::warn!(
        where_,
        input_tokens = result.stats.input_tokens,
        output_tokens = result.stats.output_tokens,
        "RLM output degraded (empty/short/failed); using chunker fallback"
    );
    RlmChunker::compress(fallback_input, target_tokens, None)
}
