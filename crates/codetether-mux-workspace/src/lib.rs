//! Workspace allocation policy for mux agent windows.
//!
//! Mux windows may either reuse the caller's checkout directly or receive a
//! managed Git worktree. Worktree creation is the isolating default, but it
//! costs a full `git worktree add` on every session and window start. Callers
//! that only need a terminal in the current checkout select [`Isolation::Shared`]
//! (`--no-worktree`) and skip that cost entirely.
//! [workspace] applies [Isolation] to a requested directory using the shared
//! worktree manager and preserves its path relative to the repository root.

mod mode;
mod worktree;

pub use mode::Isolation;

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// Resolve the workspace directory for one mux window.
///
/// # Arguments
/// * `session` - Owning mux session used in managed worktree names.
/// * `slot` - Window slot used in managed worktree names.
/// * `requested` - Existing directory to canonicalize and optionally isolate.
/// * `isolation` - Whether to share the checkout or allocate a managed worktree.
/// # Returns
/// The canonical shared directory or matching directory in a new worktree.
/// Non-repository directories are returned as canonical shared directories.
/// # Errors
/// Returns canonicalization, repository discovery or worktree creation errors.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let path = codetether_mux_workspace::workspace("work", 1, std::path::Path::new("."),
///     codetether_mux_workspace::Isolation::Shared).await?;
/// assert!(path.is_absolute());
/// # Ok(()) }
/// ```
pub async fn workspace(
    session: &str,
    slot: u64,
    requested: &Path,
    isolation: Isolation,
) -> Result<PathBuf> {
    let requested = tokio::fs::canonicalize(requested)
        .await
        .context("resolve mux workspace")?;
    if isolation.is_shared() {
        return Ok(requested);
    }
    worktree::allocate(session, slot, requested).await
}

#[cfg(test)]
mod tests;
