//! Filename-safe A2A conversation identifiers.

/// Whether a `context_id` is safe to use directly as a session id.
///
/// Accepts non-empty values of at most 128
/// bytes, and only `[A-Za-z0-9_-]` so it cannot escape the sessions
/// directory.
///
/// # Arguments
/// * `context_id` - Conversation identifier to validate.
/// # Returns
/// Whether the identifier is safe to use as a session filename.
/// # Examples
/// ```rust
/// use codetether_a2a_http::conversation::usable_as_session_id;
/// assert!(usable_as_session_id("review_42"));
/// assert!(!usable_as_session_id("../escape"));
/// ```
pub fn usable_as_session_id(context_id: &str) -> bool {
    !context_id.is_empty()
        && context_id.len() <= 128
        && !context_id.contains(|c: char| !c.is_ascii_alphanumeric() && c != '-' && c != '_')
}
