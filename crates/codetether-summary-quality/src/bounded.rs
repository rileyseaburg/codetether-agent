//! Reject unsuccessful or empty summaries before applying token limits.
use crate::{clamp_tokens, strip_stats_header};
use anyhow::{Result, bail};
use codetether_rlm::RlmResult;

/// Return a clean, budget-bounded summary from an RLM result.
/// # Arguments
/// * `result` — RLM result whose convergence and text should be validated.
/// * `target_tokens` — Requested approximate token limit.
/// # Returns
/// Clean, nonempty, bounded summary text.
/// # Errors
/// Rejects unsuccessful results and empty summary bodies.
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
