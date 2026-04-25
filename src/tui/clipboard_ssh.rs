//! Clipboard availability detection for SSH and headless sessions.

/// Check if this is an SSH session (remote connection).
///
/// Returns `true` when `SSH_CONNECTION` or `SSH_TTY` are set.
pub fn is_ssh_session() -> bool {
    std::env::var("SSH_CONNECTION").is_ok() || std::env::var("SSH_TTY").is_ok()
}

/// Check if the session is headless (no display server).
///
/// Returns `true` when `TERM` starts with `xterm` but neither `DISPLAY`
/// nor `WAYLAND_DISPLAY` is set.
pub fn is_headless_session() -> bool {
    std::env::var("TERM")
        .ok()
        .map_or(false, |t| t.starts_with("xterm"))
        && std::env::var("DISPLAY").is_err()
        && std::env::var("WAYLAND_DISPLAY").is_err()
}

/// Check if clipboard access is unavailable (SSH or headless).
///
/// Returns `true` when `SSH_CONNECTION` or `SSH_TTY` are set, or when
/// `TERM` starts with `xterm` but neither `DISPLAY` nor
/// `WAYLAND_DISPLAY` is set.
///
/// # Examples
///
/// ```rust,no_run
/// let result = codetether_agent::tui::clipboard_ssh::is_ssh_or_headless();
/// println!("Clipboard unavailable: {result}");
/// ```
pub fn is_ssh_or_headless() -> bool {
    is_ssh_session() || is_headless_session()
}

/// Build a short status message when clipboard paste is unavailable.
///
/// Returns SSH-specific guidance when connected over SSH, or a generic
/// message for other headless environments.
pub fn clipboard_unavailable_message() -> String {
    if is_ssh_session() {
        "SSH detected — clipboard not forwarded. Type ? for instructions or /image <path>."
            .to_string()
    } else {
        "Clipboard unavailable; use /image <path> to attach an image file.".to_string()
    }
}
