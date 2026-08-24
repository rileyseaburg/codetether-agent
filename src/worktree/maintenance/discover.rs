use super::{parse, record::WorktreeRecord};
use crate::worktree::WorktreeManager;
use anyhow::{Context, Result, bail};

impl WorktreeManager {
    pub(super) async fn registered_worktrees(&self) -> Result<Vec<WorktreeRecord>> {
        let output = crate::tool::git::process::output_refs(
            &self.repo_path,
            &["worktree", "list", "--porcelain", "-z"],
            false,
        )
        .await
        .context("failed to list registered Git worktrees")?;
        if !output.status.success() {
            bail!(
                "git worktree list failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Ok(parse::records(&output.stdout))
    }
}
