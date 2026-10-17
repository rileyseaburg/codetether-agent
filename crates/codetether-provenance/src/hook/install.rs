//! Install the generated commit-message hook in a repository.
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use super::{paths::repo_root, permissions::make_executable, script::commit_msg_hook_script};

/// Install the provenance commit-message script and make it executable.
///
/// # Arguments
/// * `path` - A path inside the repository to discover.
///
/// # Returns
/// Unit on success, including when no repository is found.
///
/// # Errors
/// Returns an error if Git cannot be started or the hook cannot be written or made executable.
///
/// # Examples
/// ```no_run
/// use codetether_provenance::install_commit_msg_hook;
/// install_commit_msg_hook(std::path::Path::new("."))?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn install_commit_msg_hook(path: &Path) -> Result<()> {
    let Some(root) = repo_root(path)? else {
        return Ok(());
    };
    let hook_path = root.join(".git/hooks/commit-msg");
    fs::write(&hook_path, commit_msg_hook_script()).context("Failed to write commit-msg hook")?;
    make_executable(&hook_path)?;
    Ok(())
}
