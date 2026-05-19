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

use anyhow::{Context, Result, anyhow};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

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
    exclude_legacy_worktree_helper(repo_path)?;
    let helper_path = repo_git_path(repo_path, "codetether-credential-helper")?;
    write_git_credential_helper_script(&helper_path, workspace_id)?;
    let helper_path_str = helper_path
        .to_str()
        .ok_or_else(|| anyhow!("Helper path is not valid UTF-8"))?;
    run_git_command(
        repo_path,
        &[
            "config",
            "--local",
            "--replace-all",
            "credential.helper",
            helper_path_str,
        ],
    )?;
    run_git_command(
        repo_path,
        &["config", "--local", "credential.useHttpPath", "true"],
    )?;
    run_git_command(
        repo_path,
        &["config", "--local", "codetether.workspaceId", workspace_id],
    )?;
    Ok(helper_path)
}

fn repo_git_path(repo_path: &Path, path: &str) -> Result<PathBuf> {
    let output = Command::new("git")
        .current_dir(repo_path)
        .args(["rev-parse", "--git-path", path])
        .output()
        .with_context(|| format!("Failed to resolve Git path for {}", repo_path.display()))?;
    if !output.status.success() {
        return Err(anyhow!(
            "Git path resolution failed in {}: {}",
            repo_path.display(),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    let resolved = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if resolved.is_empty() {
        return Err(anyhow!(
            "Git path resolution returned an empty path in {}",
            repo_path.display()
        ));
    }
    let path = PathBuf::from(resolved);
    Ok(if path.is_absolute() {
        path
    } else {
        repo_path.join(path)
    })
}

fn exclude_legacy_worktree_helper(repo_path: &Path) -> Result<()> {
    let exclude_path = repo_git_path(repo_path, "info/exclude")?;
    let existing = std::fs::read_to_string(&exclude_path).unwrap_or_default();
    if existing
        .lines()
        .any(|line| line.trim() == ".codetether-git-credential-helper")
    {
        return Ok(());
    }
    if let Some(parent) = exclude_path.parent() {
        std::fs::create_dir_all(parent).with_context(|| {
            format!(
                "Failed to create Git exclude directory {}",
                parent.display()
            )
        })?;
    }
    std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&exclude_path)
        .with_context(|| format!("Failed to open Git exclude file {}", exclude_path.display()))?
        .write_all(b"\n.codetether-git-credential-helper\n")
        .with_context(|| {
            format!(
                "Failed to update Git exclude file {}",
                exclude_path.display()
            )
        })
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
pub fn configure_repo_git_github_app(
    repo_path: &Path,
    installation_id: Option<&str>,
    app_id: Option<&str>,
) -> Result<()> {
    set_local_config(
        repo_path,
        "codetether.githubInstallationId",
        installation_id,
    )?;
    set_local_config(repo_path, "codetether.githubAppId", app_id)?;
    Ok(())
}
