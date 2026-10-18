//! Native specialization of shared owner-scoped hydration.
pub(crate) async fn parent(owner: Option<&str>) -> anyhow::Result<usize> {
    codetether_agent_storage::hydrate_parent::<super::host::ApplicationHost>(owner).await
}
