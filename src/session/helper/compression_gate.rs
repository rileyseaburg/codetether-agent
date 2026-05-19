//! Quality gate + chunker fallback for compaction RLM output.

use anyhow::Error;

use crate::rlm::{RlmChunker, RlmResult};
use crate::session::index_produce::summary_gate::try_bounded_summary;

/// Accept the RLM summary if it passes [`try_bounded_summary`];
/// otherwise log and fall back to deterministic chunk compression.
pub(super) fn gated_summary_or_chunk(
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

/// Compaction-wrapper around [`gated_summary_or_chunk`] that also
/// handles RLM-router errors and records telemetry on success.
pub(super) fn run_keep_last(
    routed: Result<RlmResult, Error>,
    target_tokens: usize,
    fallback_input: &str,
    reason: &str,
    rlm_model: &str,
) -> String {
    match routed {
        Ok(result) => {
            tracing::info!(
                reason,
                rlm_model = %rlm_model,
                input_tokens = result.stats.input_tokens,
                output_tokens = result.stats.output_tokens,
                compression_ratio = result.stats.compression_ratio,
                "RLM: Compressed session history"
            );
            crate::telemetry::TOKEN_USAGE.record_model_usage(
                rlm_model,
                result.stats.input_tokens as u64,
                result.stats.output_tokens as u64,
            );
            gated_summary_or_chunk(
                result,
                target_tokens,
                fallback_input,
                "compress_messages_keep_last",
            )
        }
        Err(e) => {
            tracing::warn!(reason, error = %e, "RLM: Failed to compress session history; falling back to chunker");
            RlmChunker::compress(fallback_input, target_tokens, None)
        }
    }
}
