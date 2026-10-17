//! Legacy executor trace output for storage.
use super::*;

/// Retain numbered evidence or bound other diagnostic output.
///
/// # Arguments
/// * `output` - Execution output to store.
/// # Returns
/// Original evidence or a character-limited diagnostic string.
/// # Examples
/// ```
/// assert_eq!(codetether_rlm::legacy_support::trace_output_for_storage("hello"), "hello");
/// ```
pub fn trace_output_for_storage(output: &str) -> String {
    let trimmed = output.trim();
    if trimmed == "(no matches)" || !parse_line_numbered_output(output).is_empty() {
        output.to_string()
    } else {
        truncate_with_ellipsis(output, 240)
    }
}
