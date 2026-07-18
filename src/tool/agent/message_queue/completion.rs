//! Mailbox acknowledgement after a child turn settles.

use super::{dispatcher, receipt};
use crate::tool::agent::execution_state::{self, AgentRunGuard};

pub(in crate::tool::agent) async fn finished(
    agent_id: String,
    receipt_id: Option<String>,
    guard: AgentRunGuard,
) {
    let closed = execution_state::is_closed(&agent_id);
    if let Some(receipt_id) = receipt_id {
        let result = if closed {
            receipt::release(&agent_id, &receipt_id).await
        } else {
            receipt::ack(&agent_id, &receipt_id).await
        };
        if let Err(error) = result {
            tracing::warn!(agent_id, %error, "Mailbox settlement failed");
            return;
        }
    }
    drop(guard);
    if !closed {
        dispatcher::next(agent_id);
    }
}
