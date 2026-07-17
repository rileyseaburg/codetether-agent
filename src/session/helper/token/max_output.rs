//! Session completion output-token configuration.

/// Returns the configured positive output-token limit, defaulting to 8192.
///
/// # Examples
///
/// ```rust
/// let limit = codetether_agent::session::helper::token::session_completion_max_tokens();
/// assert!(limit > 0);
/// ```
pub fn session_completion_max_tokens() -> usize {
    std::env::var("CODETETHER_SESSION_MAX_TOKENS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(8192)
}
