//! Native specialization of transactional durable-owner migration.
pub(crate) async fn owner(from: &str, to: &str) -> anyhow::Result<usize> {
    codetether_agent_storage::reparent_owner::<super::host::ApplicationHost>(from, to).await
}
