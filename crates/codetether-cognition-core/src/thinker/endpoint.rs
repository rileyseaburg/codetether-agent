//! Normalize a configured thinker base URL to the completion endpoint.

/// Ensure a thinker base URL ends at the chat-completions path.
///
/// # Arguments
/// * `base_url` - Configured thinker base URL.
///
/// # Returns
/// A normalized completion endpoint.
///
/// # Examples
/// ```
/// use codetether_cognition_core::thinker::endpoint::normalize_thinker_endpoint;
/// assert_eq!(normalize_thinker_endpoint("http://localhost/v1"), "http://localhost/v1/chat/completions");
/// ```
pub fn normalize_thinker_endpoint(base_url: &str) -> String {
    let trimmed = base_url.trim().trim_end_matches('/');
    if trimmed.ends_with("/chat/completions") {
        return trimmed.to_string();
    }
    if trimmed.is_empty() {
        return "http://127.0.0.1:11434/v1/chat/completions".to_string();
    }
    format!("{trimmed}/chat/completions")
}
