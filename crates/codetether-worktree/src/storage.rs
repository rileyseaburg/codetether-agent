//! Required storage location for managed worktrees.

use anyhow::{Result, bail};
use std::path::{Path, PathBuf};

use super::WorktreeManager;

const DIRECTORY: &str = ".codetether-worktrees";

pub(super) fn base_dir(workspace_root: &Path) -> PathBuf {
    workspace_root.join(DIRECTORY)
}

pub(super) fn validate(workspace_root: &Path, actual: &Path) -> Result<()> {
    let expected = base_dir(workspace_root);
    if actual != expected {
        bail!(
            "worktree storage must be {}; refusing {}",
            expected.display(),
            actual.display()
        );
    }
    if std::fs::symlink_metadata(actual).is_ok_and(|metadata| metadata.file_type().is_symlink()) {
        bail!("worktree storage cannot be a symlink: {}", actual.display());
    }
    Ok(())
}

impl WorktreeManager {
    pub(crate) fn validate_storage(&self) -> Result<()> {
        validate(&self.repo_path, &self.base_dir)
    }
}

#[cfg(test)]
#[path = "storage_tests.rs"]
mod tests;
