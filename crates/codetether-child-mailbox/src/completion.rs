//! Mailbox acknowledgement after a child turn settles.

use super::{dispatcher, receipt};
use crate::Host;
use codetether_collaboration_state::execution::{self as execution_state, AgentRunGuard};

/// Settle a delivery, release its run guard and dispatch the next item when open.
/// # Arguments
/// * `agent_id` — Durable child identity.
/// * `receipt_id` — Optional claimed delivery identity.
/// * `guard` — Active child execution guard.
/// # Returns
/// Unit after settlement; settlement errors are logged before returning.
/// # Examples
/// Implement [crate::Host] to provide the existing persistence and child runtime.
pub async fn finished<H: Host>(agent_id: String, receipt_id: Option<String>, guard: AgentRunGuard) {
    let closed = execution_state::is_closed(&agent_id);
    if let Some(receipt_id) = receipt_id {
        let result = if closed {
            receipt::release::<H>(&agent_id, &receipt_id).await
        } else {
            receipt::ack::<H>(&agent_id, &receipt_id).await
        };
        if let Err(error) = result {
            tracing::warn!(target:"codetether_agent::tool::agent::collaboration_runtime::message_queue::completion",agent_id, %error, "Mailbox settlement failed");
            return;
        }
    }
    drop(guard);
    if !closed {
        dispatcher::next::<H>(agent_id);
    }
}
