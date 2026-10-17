//! Canonical context-window limits for known LLM models.
//!
//! This is the **single source of truth** for
//! [`context_window_for_model`]. Every subsystem — session compaction,
//! RLM routing, TUI token badges, Bedrock estimates — must delegate here
//! rather than maintaining its own heuristic map.
//!
//! # Adding a new model
//!
//! Add a `contains` match arm below. Order matters: more specific
//! patterns must come before broader ones (e.g. `"claude-opus-4-7"`
//! before `"claude"`).
//!
//! # Examples
//!
//! ```rust
//! use codetether_provider_core::limits::context_window_for_model;
//!
//! assert_eq!(context_window_for_model("zai/glm-5"), 200_000);
//! assert_eq!(context_window_for_model("kimi-k2.5"), 256_000);
//! assert_eq!(context_window_for_model("claude-opus-4-7"), 1_000_000);
//! assert_eq!(context_window_for_model("unknown-model"), 128_000);
//! ```

#[path = "limits_million.rs"]
pub mod limits_million;

/// Return the context window size (in tokens) for known models.
///
/// Uses case-insensitive substring matching against the model identifier.
/// Returns 128 000 for unknown models (conservative for most modern LLMs).
///
/// # Arguments
/// * `model` - Model identifier, with or without a provider prefix.
///
/// # Returns
/// The estimated context capacity in tokens.
///
/// # Examples
/// ```
/// assert_eq!(codetether_provider_core::limits::context_window_for_model("unknown"), 128_000);
/// ```
pub fn context_window_for_model(model: &str) -> usize {
    let m = model.to_ascii_lowercase();

    // ── Most specific patterns first ───────────────────────────────
    if limits_million::has_million_token_context(&m) {
        1_000_000
    } else if m.contains("glm-5") || m.contains("glm5") {
        200_000
    } else if m.contains("kimi-k2") || m.contains("kimi.k2") {
        256_000
    } else if m.contains("openai.gpt-5") {
        // Bedrock-hosted GPT-5.x (5.4/5.5/5.6 Sol/Terra/Luna): 272k per
        // the Codex Bedrock catalog (GPT_5_BEDROCK_CONTEXT_WINDOW).
        272_000
    } else if m.contains("gpt-5") {
        256_000
    } else if m.contains("gpt-4o") || m.contains("gpt-4-turbo") || m.contains("gpt-4") {
        128_000
    } else if m.contains("claude") {
        200_000
    } else if m.contains("gemini-2.5-pro") || m.contains("gemini-2-pro") {
        2_000_000
    } else if m.contains("gemini") {
        1_000_000
    // ── MiniMax: most specific patterns first ────────────────────────
    } else {
        families::remaining(&m)
    }
}

#[cfg(test)]
#[path = "limits_tests.rs"]
mod tests;

mod families;
