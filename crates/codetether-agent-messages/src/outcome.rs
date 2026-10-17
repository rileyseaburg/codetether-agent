//! Runtime-error precedence and delegated completion status.

/// Preserve runtime errors and explicit non-completion; tolerate missing metadata.
/// # Arguments
/// * `response` — Final child response, optionally containing a status footer.
/// * `error` — Existing runtime failure, which takes precedence.
/// # Returns
/// The runtime error or explicit non-completion error; otherwise none.
/// # Examples
/// ```rust
/// assert!(codetether_agent_messages::effective_error("STATUS: completed", None).is_none());
/// ```
pub fn effective_error(response: &str, error: Option<String>) -> Option<String> {
    error.or_else(|| codetether_swarm_core::deliverable::deliverable_error(response))
}
