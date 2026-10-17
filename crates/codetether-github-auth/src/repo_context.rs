//! Repository context loading for bash GitHub auth.
//!
//! This module reads the current repository root, workspace id, and origin
//! remote so GitHub credentials can be requested for the correct repo.
//!
//! # Examples
//!
//! ```ignore
//! let context = load_repo_context(Some(".")).await;
//! assert!(context.is_some() || context.is_none());
//! ```

use super::command::git_stdout;
use super::remote::parse_https_remote;

/// Loads the workspace id and GitHub remote context for the current repository.
///
/// The returned tuple is `(workspace_id, host, path)` and only resolves for
/// repositories with a local `codetether.workspaceId` config entry.
///
/// # Examples
///
/// ```ignore
/// let context = load_repo_context(Some(".")).await;
/// assert!(context.is_some() || context.is_none());
/// ```
pub(super) async fn load_repo_context(cwd: Option<&str>) -> Option<(String, String, String)> {
    let repo_root = git_stdout(cwd, ["rev-parse", "--show-toplevel"]).await?;
    let workspace_id = git_stdout(
        Some(repo_root.as_str()),
        ["config", "--local", "--get", "codetether.workspaceId"],
    )
    .await?;
    let remote_url = git_stdout(Some(repo_root.as_str()), ["remote", "get-url", "origin"]).await?;
    let (host, path) = parse_https_remote(&remote_url)?;
    (host == "github.com").then_some((workspace_id, host, path))
}
