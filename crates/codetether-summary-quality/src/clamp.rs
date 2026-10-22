//! UTF-8-safe deterministic summary size limiting.
use codetether_rlm::RlmChunker;

/// Deterministically bound text to the requested approximate token budget.
/// # Arguments
/// * `text` — Summary text to bound.
/// * `target_tokens` — Requested approximate token budget.
/// # Returns
/// Original text when it fits, otherwise balanced section excerpts and a marker.
/// Late sections such as blockers and next actions receive space independently.
/// # Examples
/// ```
/// assert_eq!(codetether_summary_quality::clamp_tokens("short", 100), "short");
/// assert!(codetether_summary_quality::clamp_tokens(&"a".repeat(1000), 10).len() < 1000);
/// ```
pub fn clamp_tokens(text: &str, target_tokens: usize) -> String {
    let limit = target_tokens.saturating_mul(4).max(64);
    if RlmChunker::estimate_tokens(text) <= target_tokens || text.len() <= limit {
        return text.to_string();
    }
    let marker = "\n[summary clamped to requested token budget]";
    let keep = limit.saturating_sub(marker.len());
    let sections = super::sections::split(text);
    let bounded = codetether_rlm::text_budget::pack_sections(&sections, keep);
    format!("{}{marker}", bounded.trim_end())
}
