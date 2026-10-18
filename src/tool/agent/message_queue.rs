//! Application ownership and runtime bindings for the shared durable mailbox.
#[path = "message_queue/disk.rs"]
mod disk;
#[path = "message_queue/host.rs"]
mod host;
use codetether_child_mailbox as item;
#[path = "message_queue/params.rs"]
mod params;

pub(crate) use codetether_child_mailbox::{Submission, pending};
use host::ApplicationHost as H;
pub(in crate::tool::agent) async fn enqueue(
    agent_id: &str,
    message: String,
    input: &crate::tool::agent::params::Params,
) -> anyhow::Result<Submission> {
    super::super::store::get_for_parent(agent_id, input.parent_session_id.as_deref())
        .ok_or_else(|| anyhow::anyhow!("Agent {agent_id} not found"))?;
    codetether_child_mailbox::enqueue::<H>(
        agent_id,
        message,
        codetether_child_mailbox::EnqueueInput {
            images: &input.message_images,
            parent_id: input.parent_session_id.as_deref(),
            prior_context: input.parent_prior_context_allowed,
        },
    )
    .await
}

#[path = "message_queue/api.rs"]
mod api;
pub(in crate::tool::agent) use api::finished;
pub(crate) use api::{clear, dispatch_next, hydrate, reparent_owner};
#[cfg(test)]
#[path = "message_queue/test_api.rs"]
mod test_api;
#[cfg(test)]
pub(crate) use test_api::{ack_for_test, claim_for_test, reset_for_test};
