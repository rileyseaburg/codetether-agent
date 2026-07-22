//! Durable FIFO mailbox for follow-ups sent while a child agent is busy.

#[path = "message_queue/claim.rs"]
mod claim;
#[path = "message_queue/completion.rs"]
mod completion;
#[path = "message_queue/disk.rs"]
mod disk;
#[path = "message_queue/dispatcher.rs"]
mod dispatcher;
#[path = "message_queue/item.rs"]
mod item;
#[path = "message_queue/params.rs"]
mod params;
#[path = "message_queue/receipt.rs"]
mod receipt;
#[path = "message_queue/snapshot.rs"]
mod snapshot;
#[path = "message_queue/store.rs"]
mod store;
#[path = "message_queue/store_lifecycle.rs"]
mod store_lifecycle;
#[path = "message_queue/submission.rs"]
mod submission;

pub(in crate::tool::agent) use completion::finished;
pub(crate) use dispatcher::next as dispatch_next;
pub(crate) use store_lifecycle::reparent_owner;
pub(crate) use store_lifecycle::{clear, hydrate, pending};
pub(crate) use submission::Submission;

pub(in crate::tool::agent) async fn enqueue(
    agent_id: &str,
    message: String,
    input: &crate::tool::agent::params::Params,
) -> anyhow::Result<Submission> {
    super::super::store::get_for_parent(agent_id, input.parent_session_id.as_deref())
        .ok_or_else(|| anyhow::anyhow!("Agent {agent_id} not found"))?;
    store::enqueue(agent_id, message, input).await
}

#[cfg(test)]
pub(crate) async fn claim_for_test(agent_id: &str) -> anyhow::Result<Option<(String, String)>> {
    Ok(claim::next(agent_id)
        .await?
        .map(|item| (item.id, item.message)))
}

#[cfg(test)]
pub(crate) async fn reset_for_test() {
    store::QUEUES.lock().await.clear();
}

#[cfg(test)]
pub(crate) async fn ack_for_test(agent_id: &str, id: &str) -> anyhow::Result<()> {
    receipt::ack(agent_id, id).await
}
