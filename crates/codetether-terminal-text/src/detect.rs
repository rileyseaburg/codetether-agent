//! Detect copied terminal border artifacts.

/// Return `true` when `text` appears to contain TUI render artifacts.
/// # Arguments
/// * `text` - Copied terminal selection or pasted plain text.
/// # Returns
/// Whether any line contains a recognized terminal border character.
/// # Examples
/// ```
/// assert!(codetether_terminal_text::has_tui_artifacts("│ hello │"));
/// ```
pub fn has_tui_artifacts(text: &str) -> bool {
    text.lines()
        .any(|l| l.contains('│') || l.contains('┌') || l.contains('└'))
}
