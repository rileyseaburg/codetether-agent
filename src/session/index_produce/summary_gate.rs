//! Quality gate for RLM-produced summaries (issue #231 items 2 + 4).

use crate::rlm::RlmResult;

use super::summary_text::{clamp_tokens, strip_stats_header};

/// Quality-gated bounded summary. Returns `None` when the RLM result is
/// degraded — failure, empty body, or implausibly small output relative
/// to its input — so the caller can fall back to deterministic chunk
/// compression rather than anchoring future context with a one-liner.
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

/// Minimum plausible output-token count for an input of `input_tokens`.
///
/// Below ~256 input tokens any non-empty summary is acceptable. For
/// larger inputs we require the greater of 50 output tokens and
/// `input_tokens / 1000` (≈ 0.1 %). The ratio catches the production
/// failure mode (770 k → 180 tokens) that a flat-50 floor would miss.
pub(super) fn degraded_floor(input_tokens: usize) -> usize {
    if input_tokens < 256 {
        1
    } else {
        (input_tokens / 1000).max(50)
    }
}
