//! Bind the shared mailbox to existing atomic storage and child execution.
use super::{Execution, Mailbox, disk};
use anyhow::Result;
use codetether_child_mailbox::{Item, Snapshot};
use codetether_collaboration_state::execution::AgentRunGuard;
use codetether_tool_core::ToolResult;
#[async_trait::async_trait]
impl<E: Execution> codetether_child_mailbox::Host for Mailbox<E> {
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
        E::execute(agent_id, item, guard).await
    }
}
