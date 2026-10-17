//! Background dispatch of one claimed mailbox item.

use super::{claim, receipt};
use crate::Host;

/// Spawn dispatch for at most one pending delivery.
/// # Arguments
/// * `agent_id` — Durable child identity.
/// # Returns
/// Returns immediately after spawning the dispatch task.
/// # Examples
/// Implement [crate::Host] to provide the existing persistence and child runtime.
pub fn next<H: Host>(agent_id: String) {
    tokio::spawn(async move {
        let item = match claim::next::<H>(&agent_id).await {
            Ok(Some(item)) => item,
            Ok(None) => return,
            Err(error) => {
                tracing::warn!(target:"codetether_agent::tool::agent::collaboration_runtime::message_queue::dispatcher",agent_id, %error, "Mailbox claim failed");
                return;
            }
        };
        let Some(guard) = codetether_collaboration_state::execution::try_start(&agent_id) else {
            let _ = receipt::release::<H>(&agent_id, &item.id).await;
            return;
        };
        let receipt_id = item.id.clone();
        if let Err(error) = H::execute(agent_id.clone(), item, guard).await {
            let _ = receipt::release::<H>(&agent_id, &receipt_id).await;
            tracing::warn!(target:"codetether_agent::tool::agent::collaboration_runtime::message_queue::dispatcher",agent_id, %error, "Queued follow-up failed");
        }
    });
}
