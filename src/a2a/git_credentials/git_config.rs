//! Local Git configuration helpers.
//!
//! These helpers run small `git config` commands while keeping error reporting
//! consistent across credential-helper setup paths.
//!
//! # Examples
//!
//! ```ignore
//! run_git_command(repo_path, &["config", "--local", "credential.useHttpPath", "true"])?;
//! ```

use anyhow::{anyhow, Result};
use std::path::Path;
use std::process::Command;

/// Runs a Git command inside a repository and surfaces stderr on failure.
///
/// The command is executed with the repository as the current directory.
///
/// # Examples
///
/// ```ignore
/// run_git_command(repo_path, &["status", "--short"])?;
/// ```
pub(super) fn run_git_command(repo_path: &Path, args: &[&str]) -> Result<()> {
    let output = Command::new("git")
        .current_dir(repo_path)
        .args(args)
        .output()?;
    if output.status.success() {
        return Ok(());
    }
    Err(anyhow!(
        "Git command failed in {}: {}",
        repo_path.display(),
        String::from_utf8_lossy(&output.stderr).trim()
    ))
}

/// Sets a local Git configuration value when the value is present.
///
/// Empty or missing values are ignored so optional settings stay ergonomic.
///
/// # Examples
///
/// ```ignore
/// set_local_config(repo_path, "codetether.githubAppId", Some("123"))?;
/// ```
pub(super) fn set_local_config(
    repo_path: &Path,
    key: &str,
    value: Option<&str>,
) -> Result<()> {
    value
        .filter(|value| !value.trim().is_empty())
        .map_or(Ok(()), |value| {
            run_git_command(repo_path, &["config", "--local", key, value])
        })
}
