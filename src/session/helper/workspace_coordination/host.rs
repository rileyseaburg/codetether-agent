//! Native mux activity and acquisition adapter.
use async_trait::async_trait;
use codetether_worktree_lease::CoordinationReply;
use std::path::{Path, PathBuf};
pub(super) struct NativeHost;
#[async_trait]
impl codetether_workspace_coordination::Host for NativeHost {
    fn active(&self) -> bool {
        crate::mux::coordination::active()
    }
    async fn acquire(
        &self,
        owner: &str,
        agent: &str,
        workspace: &Path,
        paths: Vec<PathBuf>,
    ) -> anyhow::Result<Option<CoordinationReply>> {
        crate::mux::coordination::acquire(owner, agent, workspace, paths).await
    }
}
