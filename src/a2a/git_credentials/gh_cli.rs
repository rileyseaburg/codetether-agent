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

use anyhow::{Result, anyhow};
use std::io::Write;

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
pub(super) fn should_delegate_to_gh_cli(
    query: &GitCredentialQuery,
    credentials: &GitCredentialMaterial,
) -> bool {
    query
        .host
        .as_deref()
        .or(credentials.host.as_deref())
        .unwrap_or_default()
        .trim()
        .eq_ignore_ascii_case("github.com")
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
pub(super) fn emit_credentials_via_gh_cli(
    query: &GitCredentialQuery,
    credentials: &GitCredentialMaterial,
) -> Result<()> {
    let payload = render_gh_credential_query(query, credentials);
    let output = super::gh_process::run(&credentials.password, &payload)?;
    if output.status.success() {
        std::io::stdout().write_all(&output.stdout)?;
        std::io::stdout().flush()?;
        return Ok(());
    }
    Err(anyhow!(
        "gh auth git-credential failed: {}",
        String::from_utf8_lossy(&output.stderr).trim()
    ))
}
