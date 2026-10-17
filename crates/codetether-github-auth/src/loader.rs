//! Public GitHub auth loader for bash commands.
//!
//! This module coordinates command detection, repository inspection, and
//! credential loading to build the environment injected into bash executions.
//!
//! # Examples
//!
//! ```ignore
//! let auth = load_github_command_auth("gh api /user", Some(".")).await?;
//! assert!(auth.is_some());
//! ```

use super::GitHubCommandAuth;
use super::command::needs_github_auth;
use super::config::resolve_gh_config_dir;
use super::credentials::load_github_password;
use super::repo_context::load_repo_context;
use anyhow::Result;

/// Loads GitHub CLI auth for a bash command when the repo is CodeTether-managed.
///
/// The loader returns `None` for commands that do not touch GitHub or for
/// repositories that are not registered with a CodeTether workspace.
///
/// # Examples
///
/// ```ignore
/// let auth = load_github_command_auth("gh pr create", Some(".")).await?;
/// assert!(auth.is_some() || auth.is_none());
/// ```
pub async fn load_github_command_auth(
    command: &str,
    cwd: Option<&str>,
) -> Result<Option<GitHubCommandAuth>> {
    if !needs_github_auth(command) {
        return Ok(None);
    }
    let Some((workspace_id, host, path)) = load_repo_context(cwd).await else {
        return Ok(None);
    };
    let password = load_github_password(&workspace_id, host, path).await?;
    Ok(password.map(|password| GitHubCommandAuth::new(password, resolve_gh_config_dir())))
}
