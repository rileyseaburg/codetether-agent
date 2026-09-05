//! Stable managed checkout allocation for a child task.

use anyhow::{Context, Result};
use std::path::Path;

#[path = "workspace/git.rs"]
mod git;
#[path = "workspace/handoff.rs"]
mod handoff;
#[path = "workspace/paths.rs"]
mod paths;
pub(super) use handoff::Handoff;

pub(super) async fn allocate(requested: &Path) -> Result<Handoff> {
    let requested = requested
        .canonicalize()
        .context("Resolve child workspace")?;
    let root = crate::provenance::repo_root(&requested)?.context(
        "Child write isolation requires a Git checkout; refusing shared-workspace fallback",
    )?;
    let relative = requested.strip_prefix(&root)?;
    let base_commit = git::output(&root, &["rev-parse", "HEAD"]).await?;
    let primary = paths::primary(&root).await?;
    let manager = crate::worktree::WorktreeManager::for_repo(primary).without_vscode_auto_open();
    let name = format!("child-{}", uuid::Uuid::new_v4().simple());
    let info = manager.create_from(&name, &base_commit).await?;
    let handoff = Handoff {
        workspace: info.path.join(relative),
        worktree: info.path,
        branch: info.branch,
        parent_workspace: requested,
        base_commit,
    };
    manager
        .inject_workspace_stub(&handoff.worktree)
        .with_context(|| format!("Child checkout retained at {}", handoff.worktree.display()))?;
    anyhow::ensure!(
        handoff.workspace.is_dir(),
        "Child working directory is absent from committed HEAD; checkout retained at {}",
        handoff.worktree.display()
    );
    Ok(handoff)
}

#[cfg(test)]
#[path = "workspace/tests.rs"]
mod tests;
