//! Bind the shared mailbox to existing atomic storage and child execution.
use super::{disk, params};
use crate::tool::ToolResult;
use anyhow::Result;
use codetether_child_mailbox::{Item, Snapshot};
use codetether_collaboration_state::execution::AgentRunGuard;
pub(super) struct ApplicationHost;
#[async_trait::async_trait]
impl codetether_child_mailbox::Host for ApplicationHost {
    async fn load(child_id: &str) -> Result<Snapshot> {
        disk::load(child_id).await
    }
    async fn save(snapshot: &Snapshot) -> Result<()> {
        disk::save(snapshot).await
    }
    async fn remove(child_id: &str) -> Result<()> {
        disk::remove(child_id).await
    }
    async fn execute(agent_id: String, item: Item, guard: AgentRunGuard) -> Result<ToolResult> {
        let input = params::queued(&item);
        super::super::super::message::run::execute(
            agent_id,
            item.message,
            item.images,
            &input,
            guard,
            Some(item.id.clone()),
        )
        .await
    }
}
