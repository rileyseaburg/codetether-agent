//! Legacy MiniMax endpoint compatibility for Anthropic requests.

/// Normalize MiniMax base URLs to the Anthropic-compatible endpoint.
///
/// MiniMax's docs recommend using the `/anthropic` path for their
/// Anthropic-compatible harness. This rewrites the legacy `/v1` path.
///
/// # Arguments
/// * `base_url` - Configured base URL to trim and normalize.
/// # Returns
/// The legacy-endpoint replacement, or the trimmed custom URL.
/// # Examples
/// ```
/// assert_eq!(codetether_provider_anthropic::normalize_minimax_anthropic_base_url("https://api.minimax.io/v1"), "https://api.minimax.io/anthropic");
/// ```
pub fn normalize_minimax_anthropic_base_url(base_url: &str) -> String {
    let trimmed = base_url.trim().trim_end_matches('/');
    if trimmed.eq_ignore_ascii_case("https://api.minimax.io/v1") {
        "https://api.minimax.io/anthropic".to_string()
    } else {
        trimmed.to_string()
    }
}

#[cfg(test)]
#[path = "normalize_url_tests.rs"]
mod tests;
