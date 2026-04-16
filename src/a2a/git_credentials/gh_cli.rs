//! GitHub CLI delegation helpers for Git credentials.
//!
//! When the control plane returns GitHub-hosted credentials, the worker can
//! delegate the final response formatting to `gh auth git-credential`.
//!
//! # Examples
//!
//! ```ignore
//! if should_delegate_to_gh_cli(&query, &creds) { /* ... */ }
//! ```

use anyhow::{Context, Result, anyhow};
use std::process::{Command, Stdio};

use super::gh_query::render_gh_credential_query;
use super::{GitCredentialMaterial, GitCredentialQuery};

/// Returns whether GitHub CLI should handle the credential response.
///
/// Only GitHub-hosted requests are delegated to `gh`.
///
/// # Examples
///
/// ```ignore
/// assert!(should_delegate_to_gh_cli(&query, &creds));
/// ```
pub(super) fn should_delegate_to_gh_cli(query: &GitCredentialQuery, credentials: &GitCredentialMaterial) -> bool {
    query.host.as_deref().or(credentials.host.as_deref()).unwrap_or_default().trim().eq_ignore_ascii_case("github.com")
}

/// Emits Git credentials through `gh auth git-credential get`.
///
/// This preserves GitHub CLI's own output conventions for GitHub-hosted repos.
///
/// # Examples
///
/// ```ignore
/// emit_credentials_via_gh_cli(&query, &creds)?;
/// ```
pub(super) fn emit_credentials_via_gh_cli(query: &GitCredentialQuery, credentials: &GitCredentialMaterial) -> Result<()> {
    let mut child = Command::new("gh")
        .args(["auth", "git-credential", "get"])
        .env("GH_TOKEN", &credentials.password)
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::piped())
        .spawn()
        .context("Failed to spawn gh auth git-credential")?;

    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        let payload = render_gh_credential_query(query, credentials);
        stdin
            .write_all(payload.as_bytes())
            .context("Failed to write Git credential request to gh")?;
    }

    let output = child
        .wait_with_output()
        .context("Failed to read gh auth git-credential output")?;
    if output.status.success() { return Ok(()); }
    Err(anyhow!("gh auth git-credential failed: {}", String::from_utf8_lossy(&output.stderr).trim()))
}
