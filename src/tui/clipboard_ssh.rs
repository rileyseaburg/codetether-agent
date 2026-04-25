//! SSH session detection for clipboard unavailable messaging.

/// Check if the TUI is running in an SSH or headless session.
///
/// Returns `true` when `SSH_CONNECTION` or `SSH_TTY` are set, or when
/// a terminal type is set but no display server is available.
///
/// # Examples
///
/// ```rust
/// // This test runs locally, not over SSH, so it should be false.
/// let result = codetether_agent::tui::clipboard::is_ssh_or_headless();
/// assert!(!result || std::env::var("SSH_CONNECTION").is_ok());
/// ```
pub fn is_ssh_session() -> bool {
    std::env::var("SSH_CONNECTION").is_ok()
        || std::env::var("SSH_TTY").is_ok()
        || (std::env::var("TERM")
            .ok()
            .map_or(false, |t| t.starts_with("xterm"))
            && std::env::var("DISPLAY").is_err()
            && std::env::var("WAYLAND_DISPLAY").is_err())
}

/// Build a status message when clipboard paste is unavailable.
///
/// Provides a detailed SSH-specific message explaining the
/// `codetether clipboard image` bridge workflow when SSH is detected,
/// or a shorter message for other headless environments.
pub fn clipboard_unavailable_message() -> String {
    if is_ssh_session() {
        "SSH detected — clipboard not forwarded. To paste an image: \
         (1) run `codetether clipboard image` on your LOCAL machine, \
         (2) it copies a data URL to your clipboard, \
         (3) paste it here with Ctrl+Shift+V or right-click paste. \
         Or use /image <path> to attach a file from disk."
            .to_string()
    } else {
        "Clipboard unavailable; use /image <path> to attach an image file.".to_string()
    }
}
