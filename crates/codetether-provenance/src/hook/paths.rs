//! Resolve repository and commit-message paths using Git.
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Resolve the top-level directory of a Git working tree.
///
/// # Arguments
/// * `path` - Working directory for Git discovery.
///
/// # Returns
/// The discovered root, or `None` if Git reports no working tree.
///
/// # Errors
/// Returns an error if Git cannot be started.
///
/// # Examples
/// ```no_run
/// use codetether_provenance::repo_root;
/// let root = repo_root(std::path::Path::new("."))?;
/// assert!(root.is_none() || root.unwrap().is_absolute());
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn repo_root(path: &Path) -> Result<Option<PathBuf>> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(path)
        .output()
        .context("Failed to resolve git repo root")?;
    if !output.status.success() {
        return Ok(None);
    }
    let root = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok((!root.is_empty()).then(|| PathBuf::from(root)))
}

/// Resolve the repository-specific pending commit-message path.
///
/// # Arguments
/// * `repo_path` - Working directory for Git discovery.
///
/// # Returns
/// An absolute path to the commit-message file.
///
/// # Errors
/// Returns an error if Git cannot start, returns failure, or reports an empty path.
///
/// # Examples
/// ```no_run
/// use codetether_provenance::commit_editmsg_path;
/// assert!(commit_editmsg_path(std::path::Path::new("."))?.is_absolute());
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn commit_editmsg_path(repo_path: &Path) -> Result<PathBuf> {
    let output = Command::new("git")
        .args(["rev-parse", "--git-path", "COMMIT_EDITMSG"])
        .current_dir(repo_path)
        .output()
        .context("Failed to resolve COMMIT_EDITMSG path")?;
    if !output.status.success() {
        anyhow::bail!(
            "git rev-parse failed while resolving COMMIT_EDITMSG: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    absolute_path(repo_path, String::from_utf8_lossy(&output.stdout).trim())
}

fn absolute_path(repo_path: &Path, path: &str) -> Result<PathBuf> {
    if path.is_empty() {
        anyhow::bail!("git returned an empty COMMIT_EDITMSG path");
    }
    let path = PathBuf::from(path);
    Ok(if path.is_absolute() {
        path
    } else {
        repo_path.join(path)
    })
}
