//! Keyboard-only insertion into the currently focused editable input.
use crate::{Monitor, TypingError};
use std::sync::atomic::AtomicBool;

/// Send supplied plain text as real Windows Unicode keyboard events.
///
/// # Arguments
/// * `monitor` — Locally selected shared monitor containing the focused input.
/// * `text` — Supplied reply, not a prompt; no generation or clipboard is used.
/// * `cancelled` — One-way cancellation flag shared with Pause/Stop.
/// # Returns
/// Success means Windows accepted every keyboard event, not that a message
/// was submitted or that the target application stored it.
/// # Errors
/// Rejects unsupported platforms, invalid text, noneditable/password targets,
/// unavailable desktops, held modifiers, changed focus, timeout or cancellation.
/// Failure can leave partial input. Never automatically retry this operation.
/// Types directly at the current caret without a preview window or focus change.
/// No Enter/Tab/hotkey is sent; existing selection follows normal typing behavior.
/// # Examples
/// ```text
/// type_in_focused_input(&locally_selected_monitor, &queued_reply, &cancelled)?;
/// ```
pub fn type_in_focused_input(
    monitor: &Monitor,
    text: &str,
    cancelled: &AtomicBool,
) -> Result<(), TypingError> {
    if text.trim().is_empty()
        || text.encode_utf16().count() > 2000
        || text
            .chars()
            .any(|ch| ch.is_control() || matches!(ch, '\u{2028}' | '\u{2029}'))
    {
        return Err(TypingError::InvalidText);
    }
    #[cfg(windows)]
    return crate::platform::native::type_focused(monitor, text, cancelled);
    #[cfg(not(windows))]
    {
        let _ = (monitor, cancelled);
        Err(TypingError::Unsupported)
    }
}
