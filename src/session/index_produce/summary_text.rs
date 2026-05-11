//! Post-processing for RLM-produced context summaries.

use anyhow::{Result, bail};

use crate::rlm::{RlmChunker, RlmResult};

/// Return a clean, budget-bounded summary from an RLM result.
pub fn bounded_summary(result: RlmResult, target_tokens: usize) -> Result<String> {
    if !result.success {
        bail!(
            "RLM summary did not converge: {}",
            result.error.unwrap_or_else(|| "unknown reason".to_string())
        );
    }
    let clean = strip_stats_header(&result.processed);
    let trimmed = clean.trim();
    if trimmed.is_empty() {
        bail!("RLM summary produced empty body");
    }
    Ok(clamp_tokens(trimmed, target_tokens))
}

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
/// larger inputs we require **the greater of** 50 output tokens and
/// `input_tokens / 1000` (≈ 0.1 %).
///
/// The 0.1 % floor is what catches the production failure mode that
/// motivated the gate: a 770 k token input with a 180 token "summary"
/// would slip past a flat 50-token floor but is rejected by the
/// ratio (180 < 770). A real summary of 770 k input tokens should
/// land in the 1–4 k token range, so 770 is a comfortably loose lower
/// bound that still admits genuinely dense compressions.
fn degraded_floor(input_tokens: usize) -> usize {
    if input_tokens < 256 {
        return 1;
    }
    (input_tokens / 1000).max(50)
}

/// Remove the router stats banner from memory summaries.
pub fn strip_stats_header(text: &str) -> &str {
    let Some(rest) = text.strip_prefix("[RLM: ") else {
        return text;
    };
    rest.split_once("\n\n").map_or(text, |(_, body)| body)
}

/// Deterministically bound text to the requested approximate token budget.
pub fn clamp_tokens(text: &str, target_tokens: usize) -> String {
    let limit = target_tokens.saturating_mul(4).max(64);
    if RlmChunker::estimate_tokens(text) <= target_tokens || text.len() <= limit {
        return text.to_string();
    }
    let marker = "\n[summary clamped to requested token budget]";
    let keep = limit.saturating_sub(marker.len());
    let end = (0..=keep)
        .rev()
        .find(|index| text.is_char_boundary(*index))
        .unwrap_or(0);
    format!("{}{}", text[..end].trim_end(), marker)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_router_banner() {
        assert_eq!(strip_stats_header("[RLM: x]\n\nbody"), "body");
    }

    #[test]
    fn clamps_long_text() {
        assert!(clamp_tokens(&"a".repeat(1000), 10).len() < 1000);
    }

    fn rlm_result(success: bool, body: &str, input_tokens: usize, output_tokens: usize) -> RlmResult {
        RlmResult {
            processed: body.to_string(),
            stats: crate::rlm::RlmStats {
                input_tokens,
                output_tokens,
                ..Default::default()
            },
            success,
            error: None,
            trace: None,
            trace_id: None,
        }
    }

    #[test]
    fn try_bounded_rejects_unsuccessful_results() {
        let r = rlm_result(false, "ignored", 1024, 256);
        assert!(try_bounded_summary(&r, 512).is_none());
    }

    #[test]
    fn try_bounded_rejects_tiny_output_for_large_input() {
        // 770k → 12 tokens — degraded compression by any standard.
        let r = rlm_result(true, "summary", 770_000, 12);
        assert!(try_bounded_summary(&r, 512).is_none());
    }

    #[test]
    fn try_bounded_rejects_production_770k_to_180_case() {
        // The exact failure mode from the production log that motivated
        // this gate. A flat 50-token floor would let this slip through;
        // the 0.1 % ratio floor rejects it (180 < 770).
        let r = rlm_result(true, "this body is irrelevant for the count", 770_000, 180);
        assert!(try_bounded_summary(&r, 512).is_none());
    }

    #[test]
    fn try_bounded_accepts_plausible_output() {
        let body = "a real-looking summary that exceeds the degraded floor".repeat(4);
        let r = rlm_result(true, &body, 10_000, 80);
        let out = try_bounded_summary(&r, 512).unwrap();
        assert!(!out.is_empty());
    }

    #[test]
    fn try_bounded_rejects_empty_body_even_when_successful() {
        let r = rlm_result(true, "   \n\n   ", 10_000, 80);
        assert!(try_bounded_summary(&r, 512).is_none());
    }
}
