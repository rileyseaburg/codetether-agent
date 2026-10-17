//! describe for the token count tool.
use super::*;

/// Format one file's measurements; public for testing.
///
/// # Arguments
/// * `path` - Display label for the measured text.
/// * `text` - Text to measure.
///
/// # Returns
/// A line containing the estimate, byte length and line count.
///
/// # Examples
/// ```rust
/// let measured = codetether_token_count::describe("empty", "");
/// assert!(measured.ends_with("0 bytes, 0 lines"));
/// ```
pub fn describe(path: &str, text: &str) -> String {
    let tokens = RlmChunker::estimate_tokens(text);
    format!(
        "{path}: ~{tokens} tokens, {} bytes, {} lines",
        text.len(),
        text.lines().count()
    )
}
