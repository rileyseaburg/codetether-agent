//! Post-merge residual commit logic.
//!
//! After merging a worktree branch, the agent may have written files
//! to the main repo via absolute paths instead of the worktree.
//! This module detects and commits those residual changes.

use anyhow::Result;
use std::path::Path;
use std::process::Command;

/// Commit any uncommitted changes on the main branch after a merge.
///
/// Returns `true` if a commit was created.
pub fn commit_residual_changes(
    repo_path: &Path,
    story_id: &str,
    story_title: &str,
) -> Result<bool> {
    let dirty = Command::new("git")
        .args(["diff", "--quiet"])
        .current_dir(repo_path)
        .output()?;

    if dirty.status.success() {
        return Ok(false);
    }

    tracing::warn!(
        story_id = %story_id,
        "Residual uncommitted changes on main after merge"
    );

    Command::new("git")
        .args(["add", "-A"])
        .current_dir(repo_path)
        .output()?;

    let msg = format!(
        "chore({}): integrate module wiring for {}",
        story_id.to_lowercase(),
        story_title
    );

    let out = Command::new("git")
        .args(["commit", "-m", &msg])
        .current_dir(repo_path)
        .output()?;

    if out.status.success() {
        tracing::info!(story_id = %story_id, "Committed residual changes");
        Ok(true)
    } else {
        Ok(false)
    }
}
