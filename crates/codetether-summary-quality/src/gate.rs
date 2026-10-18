//! Reject degraded summary results before accepting bounded text.
use super::floor::degraded_floor;
use crate::{clamp_tokens, strip_stats_header};
use codetether_rlm::RlmResult;

/// Quality-gated bounded summary. Returns `None` when the RLM result is
/// degraded — failure, empty body, or implausibly small output relative
/// to its input — so the caller can fall back to deterministic chunk
/// compression rather than anchoring future context with a one-liner.
/// # Arguments
/// * `result` — RLM result with input/output token statistics.
/// * `target_tokens` — Requested approximate limit for accepted text.
/// # Returns
/// A bounded summary when convergence, token floor and nonempty-body checks pass.
pub fn try_bounded_summary(result: &RlmResult, target_tokens: usize) -> Option<String> {
    if !result.success {
        return None;
    }
    if result.stats.output_tokens < degraded_floor(result.stats.input_tokens) {
        return None;
    }
    let clean = strip_stats_header(&result.processed);
    let trimmed = clean.trim();
    if trimmed.is_empty() {
        return None;
    }
    Some(clamp_tokens(trimmed, target_tokens))
}
