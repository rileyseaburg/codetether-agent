//! Identify provider context-limit failures using the existing text markers.

/// Match the existing context-window error phrases.
///
/// # Arguments
/// * `err` - Error whose displayed message is inspected.
///
/// # Returns
/// Whether the error text identifies an oversized prompt.
///
/// # Examples
/// ```
/// use codetether_context_policy::request::recovery::is_prompt_too_long_error;
/// assert!(is_prompt_too_long_error(&anyhow::anyhow!("context window exceeded")));
/// assert!(!is_prompt_too_long_error(&anyhow::anyhow!("connection reset")));
/// ```
pub fn is_prompt_too_long_error(err: &anyhow::Error) -> bool {
    let msg = err.to_string().to_ascii_lowercase();
    msg.contains("prompt is too long")
        || msg.contains("context length")
        || (msg.contains("maximum context") || msg.contains("context window"))
        || (msg.contains("tokens") && msg.contains("maximum") && msg.contains("prompt"))
}
