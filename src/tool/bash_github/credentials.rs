//! GitHub credential loading for CodeTether-managed repositories.
//!
//! This module exchanges workspace metadata for a GitHub App token through the
//! A2A server so bash commands can use the same credentials as git operations.
//!
//! # Examples
//!
//! ```ignore
//! let password = load_github_password("ws", "github.com", "owner/repo").await?;
//! ```

use crate::a2a::git_credentials::{GitCredentialQuery, request_git_credentials};
use anyhow::Result;

/// Requests a GitHub token for the current CodeTether workspace.
///
/// The token comes from the A2A server's Git credential endpoint and is scoped
/// to the repository remote identified by `host` and `path`.
///
/// # Examples
///
/// ```ignore
/// let token = load_github_password("workspace-1", "github.com".into(), "owner/repo".into()).await?;
/// assert!(token.is_some() || token.is_none());
/// ```
pub(super) async fn load_github_password(
    workspace_id: &str,
    host: String,
    path: String,
) -> Result<Option<String>> {
    let Ok(server) = std::env::var("CODETETHER_SERVER") else {
        return Ok(None);
    };
    let credentials = request_git_credentials(
        &server,
        &std::env::var("CODETETHER_TOKEN").ok(),
        std::env::var("CODETETHER_WORKER_ID").ok().as_deref(),
        workspace_id,
        "get",
        &GitCredentialQuery {
            protocol: Some("https".to_string()),
            host: Some(host),
            path: Some(path),
        },
    )
    .await?;
    Ok(credentials.map(|credentials| credentials.password))
}
