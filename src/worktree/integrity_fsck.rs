//! Sandboxed repository object-integrity probe.

use super::WorktreeManager;
use anyhow::{Context, Result};

impl WorktreeManager {
    pub(crate) async fn run_repo_fsck(&self) -> Result<std::process::Output> {
        crate::tool::git::process::output_refs(
            &self.repo_path,
            &["fsck", "--full", "--no-dangling"],
            false,
        )
        .await
        .context("Failed to execute git fsck --full --no-dangling")
    }
}
