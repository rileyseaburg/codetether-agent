use super::gated_summary_or_chunk;
use anyhow::Error;
use codetether_rlm::{RlmChunker, RlmResult};
/// Compaction-wrapper around [`gated_summary_or_chunk`] that also
/// handles RLM-router errors and records telemetry on success.
///
/// # Arguments
/// * `routed` - RLM success or routing error.
/// * `target_tokens`, `fallback_input` - Token budget and fallback source.
/// * `reason`, `rlm_model` - Logging context and model attribution.
/// * `record_usage` - Called once on success, before validating the summary.
/// # Returns
/// Accepted RLM summary or deterministic chunk fallback on errors or low quality.
/// # Examples
/// ```text
/// let summary = run_keep_last(routed, 512, transcript, "budget", model, record_usage);
/// ```

pub fn run_keep_last(
    routed: Result<RlmResult, Error>,
    target_tokens: usize,
    fallback_input: &str,
    reason: &str,
    rlm_model: &str,
    record_usage: impl FnOnce(&str, u64, u64),
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
            record_usage(
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
