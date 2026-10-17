//! Application host specialization for shared mailbox operations.
use super::H;
use crate::tool::agent::execution_state::AgentRunGuard;

pub(in crate::tool::agent) async fn finished(
    agent_id: String,
    receipt_id: Option<String>,
    guard: AgentRunGuard,
) {
    codetether_child_mailbox::finished::<H>(agent_id, receipt_id, guard).await
}

pub(crate) fn dispatch_next(agent_id: String) {
    codetether_child_mailbox::dispatch_next::<H>(agent_id)
}

pub(crate) async fn hydrate(agent_id: &str) -> anyhow::Result<bool> {
    codetether_child_mailbox::hydrate::<H>(agent_id).await
}

pub(crate) async fn clear(agent_id: &str) -> anyhow::Result<()> {
    codetether_child_mailbox::clear::<H>(agent_id).await
}

pub(crate) async fn reparent_owner(agent_id: &str, from: &str, to: &str) -> anyhow::Result<()> {
    codetether_child_mailbox::reparent_owner::<H>(agent_id, from, to).await
}
