//! SSH and display-environment hints for terminal guidance.
//! Clipboard availability detection for SSH and headless sessions.

/// Check if this is an SSH session (remote connection).
/// # Arguments
/// None; inspects SSH environment variables.
/// # Returns
/// Whether SSH_CONNECTION or SSH_TTY is present.
/// # Examples
/// ```no_run
/// let _value = codetether_terminal_clipboard::is_ssh_session();
/// ```
pub fn is_ssh_session() -> bool {
    std::env::var("SSH_CONNECTION").is_ok() || std::env::var("SSH_TTY").is_ok()
}

/// Check if the session is headless (no display server).
#[cfg(target_os = "linux")]
/// # Arguments
/// None; inspects platform display environment.
/// # Returns
/// Whether Linux has neither X11 nor Wayland display configuration.
/// # Examples
/// ```no_run
/// let _value = codetether_terminal_clipboard::is_headless_session();
/// ```
pub fn is_headless_session() -> bool {
    std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err()
}

/// Non-Linux platforms can expose a native clipboard without display variables.
#[cfg(not(target_os = "linux"))]
/// # Arguments
/// None; inspects platform display environment.
/// # Returns
/// Whether Linux has neither X11 nor Wayland display configuration.
/// # Examples
/// ```no_run
/// let _value = codetether_terminal_clipboard::is_headless_session();
/// ```
pub fn is_headless_session() -> bool {
    false
}

/// Check if clipboard access is unavailable (SSH or headless).
/// # Arguments
/// None; combines SSH and display hints.
/// # Returns
/// Whether terminal guidance should offer alternative clipboard methods.
/// # Examples
/// ```no_run
/// let _value = codetether_terminal_clipboard::is_ssh_or_headless();
/// ```
pub fn is_ssh_or_headless() -> bool {
    is_ssh_session() || is_headless_session()
}
