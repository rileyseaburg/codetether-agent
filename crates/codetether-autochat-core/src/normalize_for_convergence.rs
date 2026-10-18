//! Shared auto-chat request policy.

/// Normalize ASCII text for bounded convergence comparisons.
///
/// # Arguments
/// * `text` - Candidate turn output.
/// * `max_chars` - Maximum normalized ASCII length before stopping.
///
/// # Returns
/// Lowercase alphanumeric text with collapsed whitespace and symbols removed.
///
/// # Examples
/// ```
/// use codetether_autochat_core::normalize_for_convergence;
/// assert_eq!(normalize_for_convergence("Done!", 20), "done");
/// ```
pub fn normalize_for_convergence(text: &str, max_chars: usize) -> String {
    let mut normalized = String::with_capacity(text.len().min(512));
    let mut last_was_space = false;

    for ch in text.chars() {
        if ch.is_ascii_alphanumeric() {
            normalized.push(ch.to_ascii_lowercase());
            last_was_space = false;
        } else if ch.is_whitespace() && !last_was_space {
            normalized.push(' ');
            last_was_space = true;
        }

        if normalized.len() >= max_chars {
            break;
        }
    }

    normalized.trim().to_string()
}
