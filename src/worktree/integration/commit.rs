use crate::provenance::{
    ExecutionOrigin, ExecutionProvenance, git_commit_with_provenance_blocking,
};
use anyhow::{Result, bail};
use std::path::Path;

pub(super) fn create(repo: &Path, branch: &str) -> Result<()> {
    let message = format!("Merge branch '{branch}' through verified integration");
    let provenance = ExecutionProvenance::for_operation("worktree", ExecutionOrigin::LocalCli);
    let output = git_commit_with_provenance_blocking(repo, &message, Some(&provenance))?;
    if !output.status.success() {
        bail!(
            "integration commit failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}
