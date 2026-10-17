//! Ownership-aware native mailbox operations used by message routing.
use anyhow::Result;
use async_trait::async_trait;
use codetether_agent_input::Params;
use codetether_child_mailbox::Submission;
/// Native durable mailbox submission and dispatch.
/// # Implementors
/// Applications preserve their existing owner checks and mailbox adapters.
#[async_trait]
pub trait MailboxHost {
    /// Enqueue one message with its runtime context, preserving submission errors.
    async fn enqueue(agent_id: &str, message: String, params: &Params) -> Result<Submission>;
    /// Trigger processing of the next pending mailbox item.
    fn dispatch_next(agent_id: String);
}
