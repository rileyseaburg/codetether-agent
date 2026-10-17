//! normalize_for_relay formatting.
/// Render normalize_for_relay output.
///
/// # Arguments
/// * `text` - Relay output to normalize.
/// # Returns
/// Lowercase ASCII words with normalized whitespace, bounded at 280 bytes.
/// # Examples
/// ```
/// assert_eq!(codetether_ralph_core::presentation::normalize_for_relay("Hello, WORLD!"), "hello world");
/// ```

/// Normalize text for convergence detection in relay loops.
pub fn normalize_for_relay(text: &str) -> String {
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
        if normalized.len() >= 280 {
            break;
        }
    }
    normalized.trim().to_string()
}
