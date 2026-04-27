//! Clipboard availability detection for SSH and headless sessions.

/// Check if this is an SSH session (remote connection).
pub fn is_ssh_session() -> bool {
    std::env::var("SSH_CONNECTION").is_ok() || std::env::var("SSH_TTY").is_ok()
}

/// Check if the session is headless (no display server).
pub fn is_headless_session() -> bool {
    std::env::var("TERM")
        .ok()
        .map_or(false, |t| t.starts_with("xterm"))
        && std::env::var("DISPLAY").is_err()
        && std::env::var("WAYLAND_DISPLAY").is_err()
}

/// Check if clipboard access is unavailable (SSH or headless).
pub fn is_ssh_or_headless() -> bool {
    is_ssh_session() || is_headless_session()
}

/// Build a status message when clipboard paste is unavailable.
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
