//! Connection services for the authoritative workspace coordinator.
use async_trait::async_trait;
use codetether_worktree_lease::CoordinationReply;
use std::path::{Path, PathBuf};
/// Supplies coordinator discovery and an authenticated lease-acquisition call.
/// # Implementors
/// Applications retain mux identity, transport and authentication ownership.
#[async_trait]
pub trait Host: Sync {
    /// Whether this invocation runs under an inherited coordinator.
    fn active(&self) -> bool;
    /// Acquire bounded paths for the trusted owner and agent identity.
    /// # Errors
    /// Propagates discovery, transport and coordinator request failures.
    async fn acquire(
        &self,
        owner: &str,
        agent: &str,
        workspace: &Path,
        paths: Vec<PathBuf>,
    ) -> anyhow::Result<Option<CoordinationReply>>;
}
