//! Cached-summary text for an oversized original message.

/// Wrap a cached summary with the original token count and request prefix.
///
/// # Arguments
/// * `summary` - Cached background summary.
/// * `original` - Original request text.
/// * `msg_tokens` - Estimated original token count.
/// * `prefix_chars` - Maximum Unicode characters retained from the original.
///
/// # Returns
/// Formatted replacement text.
///
/// # Examples
/// ```
/// use codetether_context_policy::compression::summary::wrap_cached;
/// let text = wrap_cached("summary".into(), "hello", 100, 3);
/// assert!(text.ends_with("hel"));
/// ```

pub fn wrap_cached(
    summary: String,
    original: &str,
    msg_tokens: usize,
    prefix_chars: usize,
) -> String {
    let prefix: String = original.chars().take(prefix_chars).collect();
    format!(
        "[Original message: {msg_tokens} tokens, compressed via background RLM]\n\n{summary}\n\n---\nOriginal request prefix:\n{prefix}"
    )
}
