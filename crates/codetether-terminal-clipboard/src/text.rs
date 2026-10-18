//! Copy through the system clipboard, falling back to terminal OSC52.
//! Text clipboard helpers: system clipboard + OSC52 escape sequence.
use crate::{osc52::osc52_copy, platform::platform_set_text};

/// Copy text to the system clipboard, with OSC52 as a secondary option.
/// Returns the method name on success.
/// # Arguments
/// * `text` - Nonempty text to copy.
/// # Returns
/// The successful method name.
/// # Errors
/// Returns an error for empty text or a failed OSC52 output operation.
/// # Examples
/// ```no_run
/// let _method = codetether_terminal_clipboard::copy_text("hello");
/// ```
pub fn copy_text(text: &str) -> Result<&'static str, String> {
    if text.trim().is_empty() {
        return Err("empty text".to_string());
    }
    match platform_set_text(text) {
        Ok(()) => return Ok("system clipboard"),
        Err(e) => {
            tracing::debug!(error = %e, "clipboard write failed, trying OSC52");
        }
    }
    osc52_copy(text).map_err(|e| format!("osc52: {e}"))?;
    Ok("OSC52")
}
