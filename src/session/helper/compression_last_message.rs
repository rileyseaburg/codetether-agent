//! Background summary wrapper for the oversized-last-message path.

pub(super) fn wrap_cached(
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
