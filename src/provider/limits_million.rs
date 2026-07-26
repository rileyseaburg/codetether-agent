//! Predicate for models with a 1M-token context window.
//!
//! Extracted from [`super::limits`] so the frontier-context list can grow
//! without inflating the main dispatch chain.
//!
//! # Examples
//!
//! ```rust
//! use codetether_agent::provider::limits::limits_million::has_million_token_context;
//!
//! assert!(has_million_token_context("claude-opus-5"));
//! assert!(has_million_token_context("claude-opus-4-7"));
//! assert!(!has_million_token_context("claude-opus-4-6"));
//! ```

/// Whether the (lowercased) model ID has a 1M-token context window.
///
/// # Arguments
///
/// * `m` — Model identifier, already lowercased by the caller.
///
/// # Returns
///
/// `true` for Claude Opus 5, Opus 4.7, and GLM 5.2 family IDs.
pub fn has_million_token_context(m: &str) -> bool {
    let m = m.to_ascii_lowercase();
    m.contains("claude-opus-5")
        || m.contains("claude-opus-4-7")
        || m.contains("claude-opus-4.7")
        || m.contains("4.7-opus")
        || m.contains("glm-5.2")
        || m.contains("glm5.2")
}

#[cfg(test)]
#[path = "limits_million_tests.rs"]
mod tests;
