//! SSH session detection for audio transport selection.
//!
//! Detects the remote-login case that matters for voice: a Windows
//! operator connected over SSH to a remote Ubuntu host.

/// Environment variables OpenSSH sets for an interactive login.
///
/// `SSH_CONNECTION` and `SSH_CLIENT` are set by the server for the
/// login shell; `SSH_TTY` only appears when a PTY was allocated.
const SSH_MARKERS: [&str; 3] = ["SSH_CONNECTION", "SSH_CLIENT", "SSH_TTY"];

/// True when any OpenSSH session marker is present and non-empty.
///
/// # Arguments
/// * `lookup` - Returns a variable value by name, or None if absent.
///
/// # Returns
/// Whether an OpenSSH marker has a nonblank value.
///
/// # Examples
///
/// ```rust
/// use codetether_voice_core::ssh_env::is_ssh_session_from;
///
/// assert!(is_ssh_session_from(|k| {
///     (k == "SSH_CONNECTION").then(|| "10.0.0.4 51234 10.0.0.9 22".to_string())
/// }));
/// assert!(!is_ssh_session_from(|_| None));
/// // Present but empty must not count as SSH.
/// assert!(!is_ssh_session_from(|_| Some(String::new())));
/// ```
pub fn is_ssh_session_from<F>(lookup: F) -> bool
where
    F: Fn(&str) -> Option<String>,
{
    SSH_MARKERS
        .iter()
        .filter_map(|key| lookup(key))
        .any(|value| !value.trim().is_empty())
}

/// True when this process is running inside an SSH session.
///
/// # Arguments
/// None; reads OpenSSH markers from the process environment.
///
/// # Returns
/// Whether any marker has a nonblank value.
///
/// # Examples
/// ```no_run
/// use codetether_voice_core::ssh_env::is_ssh_session;
/// let _remote: bool = is_ssh_session();
/// ```
pub fn is_ssh_session() -> bool {
    is_ssh_session_from(|key| std::env::var(key).ok())
}

#[cfg(test)]
#[path = "ssh_env_tests.rs"]
mod tests;
