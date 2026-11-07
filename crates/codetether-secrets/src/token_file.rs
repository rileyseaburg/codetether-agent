//! Vault token read from an externally rotated file (`VAULT_TOKEN_FILE`).
//!
//! A workstation `vault agent` keeps a valid periodic token in a 0600 sink
//! file and rotates it. Reading that file, instead of a token copied into a
//! service environment file, keeps long-running workers authenticated across
//! rotations. The token is never logged.

use anyhow::{Context, Result, ensure};
use std::path::{Path, PathBuf};

/// Environment variable naming the token file; it overrides `VAULT_TOKEN`.
pub(crate) const VAULT_TOKEN_FILE: &str = "VAULT_TOKEN_FILE";

/// Location of a Vault token that another process keeps current.
#[derive(Debug, Clone)]
pub(crate) struct TokenFileAuth {
    pub(crate) address: String,
    pub(crate) path: PathBuf,
}

impl TokenFileAuth {
    /// Returns the configured token file, if `VAULT_TOKEN_FILE` is set and nonempty.
    pub(crate) fn from_env(address: &str) -> Option<Self> {
        let path = std::env::var_os(VAULT_TOKEN_FILE)?;
        if path.is_empty() {
            return None;
        }
        Some(Self {
            address: address.to_string(),
            path: path.into(),
        })
    }
}

/// Reads and trims the token stored in `path`.
///
/// # Errors
/// Returns an error when the file cannot be read or contains only whitespace.
/// The error names the file but never includes its contents.
pub(crate) async fn read_token(path: &Path) -> Result<String> {
    let text = tokio::fs::read_to_string(path)
        .await
        .with_context(|| format!("Failed to read Vault token file {}", path.display()))?;
    let token = text.trim();
    ensure!(
        !token.is_empty(),
        "Vault token file {} is empty",
        path.display()
    );
    Ok(token.to_string())
}

#[cfg(test)]
mod tests;
