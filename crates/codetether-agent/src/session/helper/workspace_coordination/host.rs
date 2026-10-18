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

#[async_trait]
impl codetether_workspace_coordination::LeaseHost for NativeHost {
    fn active(&self) -> bool {
        crate::mux::coordination::active()
    }
    async fn renew(&self, owner: &str) -> anyhow::Result<Option<CoordinationReply>> {
        crate::mux::coordination::renew(owner).await
    }
    async fn release(&self, owner: &str) -> anyhow::Result<Option<CoordinationReply>> {
        crate::mux::coordination::release(owner).await
    }
}
