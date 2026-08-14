//! Workspace allocation policy for mux agent windows.
//!
//! Mux windows may either reuse the caller's checkout directly or receive a
//! managed Git worktree. Worktree creation is the isolating default, but it
//! costs a full `git worktree add` on every session and window start. Callers
//! that only need a terminal in the current checkout select [`Isolation::Shared`]
//! (`--no-worktree`) and skip that cost entirely.

mod mode;
mod worktree;

pub(crate) use mode::Isolation;

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// Resolve the workspace directory for one mux window.
pub(crate) async fn workspace(
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
