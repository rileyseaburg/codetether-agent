//! Native session specialization of shared manifest lifecycle transitions.
use crate::session::Session;
use crate::tool::agent::store::AgentEntry;
use anyhow::Result;
pub(in crate::tool::agent) type ClosedAgent = codetether_agent_storage::ClosedAgent<Session>;
pub(in crate::tool::agent) async fn close(name: &str, entry: &AgentEntry) -> Result<()> {
    codetether_agent_storage::close(name, entry).await
}
pub(in crate::tool::agent) async fn prepare_resume(
    owner: Option<&str>,
    target: &str,
) -> Result<ClosedAgent> {
    codetether_agent_storage::prepare_resume::<super::host::ApplicationHost>(owner, target).await
}
pub(in crate::tool::agent) async fn activate_resume(closed: ClosedAgent) -> Result<AgentEntry> {
    codetether_agent_storage::activate_resume::<super::host::ApplicationHost>(closed).await
}
pub(in crate::tool::agent) use codetether_agent_storage::exists;
