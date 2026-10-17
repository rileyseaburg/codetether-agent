//! Allocate one checkout from the requested repository's committed HEAD.
use crate::{Handoff, git, paths};
use anyhow::{Context, Result};
use std::path::Path;
/// Allocate a managed child checkout while preserving relative workspace cwd.
/// # Arguments
/// * `requested` — Existing working directory inside a Git checkout.
/// # Returns
/// The child checkout identity and explicit integration handoff.
/// # Errors
/// Returns canonicalization, Git, checkout-creation or workspace-stub errors.
/// Rejects unsupported repository layouts and cwd absent from committed HEAD.
/// Once created, a failed checkout is retained and its path is reported.
/// # Examples
/// ```rust,no_run
/// # async fn example() -> anyhow::Result<()> {
/// let child = codetether_child_workspace::allocate(std::path::Path::new("/repo")).await?;
/// assert!(child.workspace.is_dir());
/// # Ok(()) }
/// ```

pub async fn allocate(requested: &Path) -> Result<Handoff> {
    let requested = requested
        .canonicalize()
        .context("Resolve child workspace")?;
    let root = codetether_provenance::repo_root(&requested)?.context(
        "Child write isolation requires a Git checkout; refusing shared-workspace fallback",
    )?;
    let relative = requested.strip_prefix(&root)?;
    let base_commit = git::output(&root, &["rev-parse", "HEAD"]).await?;
    let primary = paths::primary(&root).await?;
    let manager =
        codetether_worktree::WorktreeManager::for_repo(primary).without_vscode_auto_open();
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
