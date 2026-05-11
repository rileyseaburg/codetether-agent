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
    Ok(clamp_tokens(clean.trim(), target_tokens))
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
}
