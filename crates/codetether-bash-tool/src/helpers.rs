//! Output redaction, authentication detection, and recursive command setup.
use std::ffi::OsString;

/// Recognize password prompt diagnostics in command output.
///
/// # Arguments
/// `output` is captured command output.
/// # Returns
/// Whether the output resembles an interactive authentication request.
/// # Examples
/// ```rust
/// assert!(codetether_bash_tool::looks_like_auth_prompt("password:"));
/// ```
pub fn looks_like_auth_prompt(output: &str) -> bool {
    let lower = output.to_ascii_lowercase();
    [
        "[sudo] password for",
        "password:",
        "passphrase",
        "no tty present and no askpass program specified",
        "a terminal is required to read the password",
        "permission denied (publickey,password",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
}

pub(super) fn redact_output(mut output: String, secrets: &[String]) -> String {
    for secret in secrets {
        if !secret.is_empty() {
            output = output.replace(secret, "[REDACTED]");
        }
    }
    output
}

/// Wrap shell text so nested CodeTether calls resolve the running binary.
///
/// # Arguments
/// `command` is the shell text to execute.
/// # Returns
/// Shell text prefixed with the CodeTether function definition.
/// # Examples
/// ```rust
/// assert!(codetether_bash_tool::codetether_wrapped_command("echo hi").ends_with("echo hi"));
/// ```
pub fn codetether_wrapped_command(command: &str) -> String {
    format!(
        "codetether() {{ \"$CODETETHER_BIN\" \"$@\"; }}\nexport -f codetether >/dev/null 2>&1 || true\n{command}"
    )
}

pub(super) fn codetether_runtime_env() -> Option<(String, OsString)> {
    let current_exe = std::env::current_exe().ok()?;
    let mut path_entries = current_exe
        .parent()
        .map(|parent| vec![parent.to_path_buf()])
        .unwrap_or_default();
    if let Some(existing_path) = std::env::var_os("PATH") {
        path_entries.extend(std::env::split_paths(&existing_path));
    }
    let path = std::env::join_paths(path_entries).ok()?;
    Some((current_exe.to_string_lossy().into_owned(), path))
}
