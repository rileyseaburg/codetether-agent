//! Inactive coordinator used only to inspect cancellation ownership.
pub(super) struct InactiveHost;
#[async_trait::async_trait]
impl crate::LeaseHost for InactiveHost {
    fn active(&self) -> bool {
        false
    }
    async fn renew(
        &self,
        _: &str,
    ) -> anyhow::Result<Option<codetether_worktree_lease::CoordinationReply>> {
        Ok(None)
    }
    async fn release(
        &self,
        _: &str,
    ) -> anyhow::Result<Option<codetether_worktree_lease::CoordinationReply>> {
        Ok(None)
    }
}
