//! Clipboard guidance for unavailable local or forwarded clipboards.
use crate::environment::is_ssh_session;

/// Build a status message when clipboard paste is unavailable.
/// # Arguments
/// None; inspects the current SSH environment.
/// # Returns
/// User-facing clipboard recovery instructions.
/// # Examples
/// ```no_run
/// let _value = codetether_terminal_clipboard::clipboard_unavailable_message();
/// ```
pub fn clipboard_unavailable_message() -> String {
    if is_ssh_session() {
        "SSH detected — clipboard not forwarded. To paste an image: \
         (1) run `codetether clipboard image` on your LOCAL machine, \
         (2) it copies a data URL to your clipboard, \
         (3) paste it here with Ctrl+Shift+V or right-click paste. \
         Or use /image <path> to attach a file from disk."
            .to_string()
    } else {
        "System clipboard unavailable — use Ctrl+Shift+V or right-click paste for text, \
         or /image <path> to attach an image file."
            .to_string()
    }
}
