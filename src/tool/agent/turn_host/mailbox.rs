//! Native ownership-aware mailbox submission and dispatch.
use super::ApplicationHost;
use crate::tool::agent::collaboration_runtime::message_queue;
use anyhow::Result;
use async_trait::async_trait;
use codetether_agent_input::Params;
use codetether_child_mailbox::Submission;
use codetether_child_turn::MailboxHost;
#[async_trait]
impl MailboxHost for ApplicationHost {
    async fn enqueue(agent_id: &str, message: String, params: &Params) -> Result<Submission> {
        message_queue::enqueue(agent_id, message, params).await
    }
    fn dispatch_next(agent_id: String) {
        message_queue::dispatch_next(agent_id);
    }
}
