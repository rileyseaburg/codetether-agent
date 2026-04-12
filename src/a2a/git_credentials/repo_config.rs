//! Repository-local Git credential configuration.
//!
//! This module wires repositories to the generated helper script and stores
//! GitHub App metadata used by downstream Git operations.
//!
//! # Examples
//!
//! ```ignore
//! let helper = configure_repo_git_auth(repo_path, "ws-1")?;
//! ```

use anyhow::{Result, anyhow};
use std::path::{Path, PathBuf};

use super::git_config::{run_git_command, set_local_config};
use super::write_git_credential_helper_script;

/// Configures a repository to use the CodeTether Git credential helper.
///
/// The helper path is returned so callers can inspect or clean it up later.
///
/// # Examples
///
/// ```ignore
/// let helper = configure_repo_git_auth(repo_path, "ws-1")?;
/// ```
pub fn configure_repo_git_auth(repo_path: &Path, workspace_id: &str) -> Result<PathBuf> {
    let helper_path = repo_path.join(".codetether-git-credential-helper");
    write_git_credential_helper_script(&helper_path, workspace_id)?;
    run_git_command(repo_path, ["config", "--local", "credential.helper", helper_path.to_str().ok_or_else(|| anyhow!("Helper path is not valid UTF-8"))?])?;
    run_git_command(repo_path, ["config", "--local", "credential.useHttpPath", "true"])?;
    run_git_command(repo_path, ["config", "--local", "codetether.workspaceId", workspace_id])?;
    Ok(helper_path)
}

/// Stores GitHub App identifiers in repository-local Git config.
///
/// Empty values are ignored so callers can pass optional server metadata.
///
/// # Examples
///
/// ```ignore
/// configure_repo_git_github_app(repo_path, Some("1"), Some("2"))?;
/// ```
pub fn configure_repo_git_github_app(repo_path: &Path, installation_id: Option<&str>, app_id: Option<&str>) -> Result<()> {
    set_local_config(repo_path, "codetether.githubInstallationId", installation_id)?;
    set_local_config(repo_path, "codetether.githubAppId", app_id)?;
    Ok(())
}
