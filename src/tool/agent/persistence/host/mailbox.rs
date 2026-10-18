//! Native durable-mailbox effects during recovery and ownership transfer.
use super::ApplicationHost;
use crate::tool::agent::collaboration_runtime::message_queue;
use anyhow::Result;
use async_trait::async_trait;
use codetether_agent_storage::Host;
#[async_trait]
impl Host for ApplicationHost {
    async fn hydrate_mailbox(id: &str) -> Result<bool> {
        message_queue::hydrate(id).await
    }
    fn dispatch_next(id: String) {
        message_queue::dispatch_next(id);
    }
    async fn reparent_mailbox(id: &str, from: &str, to: &str) -> Result<()> {
        message_queue::reparent_owner(id, from, to).await
    }
}
