//! Application turn execution using the native durable mailbox host.
use super::params;
use anyhow::Result;
use codetether_child_mailbox::Item;
use codetether_collaboration_state::execution::AgentRunGuard;
use codetether_tool_core::ToolResult;
pub(super) type ApplicationHost = codetether_native_agent::Mailbox<Execution>;
pub(super) struct Execution;
#[async_trait::async_trait]
impl codetether_native_agent::MailboxExecution for Execution {
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
