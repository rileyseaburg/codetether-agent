//! Thresholds for replacing long input with compact placeholders.

/// Threshold above which a paste is summarised instead of being
/// inlined into the input buffer.
///
/// Multi-line snippets shorter than this are still useful to see
/// inline (short addresses, two-line commands, etc.). Anything
/// longer becomes visual noise that hides the user's actual prompt
/// text.
const PASTE_SUMMARIZE_LINES: usize = 5;

const PASTE_SUMMARIZE_BYTES: usize = 400;

/// Decide whether a paste is large enough to warrant summarisation.
///
/// # Arguments
/// * `text` — Pasted content.
///
/// # Returns
/// Whether the line or byte threshold is reached.
///
/// # Examples
/// ```
/// assert!(codetether_paste_buffer::should_summarize(&"x".repeat(400)));
/// ```
pub fn should_summarize(text: &str) -> bool {
    let lines = text.lines().count();
    let bytes = text.len();
    lines >= PASTE_SUMMARIZE_LINES || bytes >= PASTE_SUMMARIZE_BYTES
}
