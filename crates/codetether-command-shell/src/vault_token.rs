//! Give agent shell commands the current Vault token.
//!
//! A long-running worker inherits whatever `VAULT_TOKEN` its service
//! environment held at start, which goes stale when the token is revoked or
//! expires. A workstation `vault agent` keeps a valid token in a rotated sink
//! file named by `VAULT_TOKEN_FILE`. Reading that file each time a command is
//! spawned lets the agent's own `vault` calls authenticate with a live token,
//! the same way an operator's shell does. The token is never logged.

use std::ffi::OsStr;
use std::path::Path;
use tokio::process::Command;

/// Environment variable naming the externally rotated token file.
const VAULT_TOKEN_FILE: &str = "VAULT_TOKEN_FILE";

/// Export the token stored in `VAULT_TOKEN_FILE` as the command's `VAULT_TOKEN`.
///
/// Does nothing when the variable is unset or empty, or when the file is
/// unreadable or blank; the command then keeps its inherited environment.
///
/// # Arguments
/// * `cmd` - Command to configure before spawning.
/// # Returns
/// Updates the command in place.
/// # Examples
/// ```rust
/// let mut cmd = tokio::process::Command::new("example");
/// codetether_command_shell::apply_vault_token(&mut cmd);
/// ```
pub fn apply_vault_token(cmd: &mut Command) {
    apply_from(cmd, std::env::var_os(VAULT_TOKEN_FILE).as_deref());
}

/// Apply the token from `token_file`, when one is configured and usable.
pub(crate) fn apply_from(cmd: &mut Command, token_file: Option<&OsStr>) {
    let Some(path) = token_file.filter(|path| !path.is_empty()) else {
        return;
    };
    if let Some(token) = read_token(Path::new(path)) {
        cmd.env("VAULT_TOKEN", token);
    }
}

/// Read and trim the token in `path`; `None` when unreadable or blank.
fn read_token(path: &Path) -> Option<String> {
    let text = std::fs::read_to_string(path).ok()?;
    let token = text.trim();
    (!token.is_empty()).then(|| token.to_string())
}

#[cfg(test)]
mod tests;
