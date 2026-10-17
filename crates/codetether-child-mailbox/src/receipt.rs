//! Durable acknowledgement and release of claimed mailbox items.

use super::{
    item::{DeliveryState, Item},
    store::QUEUES,
};
use crate::Host;
use anyhow::{Result, bail};

/// Remove and persist the matching front delivery.
/// # Arguments
/// * `agent_id` — Durable child identity.
/// * `id` — Receipt expected at the queue front.
/// # Returns
/// Unit after acknowledgement and completion notification.
/// # Errors
/// Returns missing/mismatched receipt errors or persistence errors; failed writes restore the item as pending.
/// # Examples
/// Implement [crate::Host] to provide the existing persistence and child runtime.
pub async fn ack<H: Host>(agent_id: &str, id: &str) -> Result<()> {
    let mut queues = QUEUES.lock().await;
    let snapshot = queues
        .get_mut(agent_id)
        .ok_or_else(|| anyhow::anyhow!("mailbox missing"))?;
    if snapshot.items.front().is_none_or(|item| item.id != id) {
        bail!("mailbox receipt mismatch");
    }
    let item = snapshot.items.pop_front().expect("receipt checked");
    if let Err(error) = H::save(snapshot).await {
        snapshot.items.push_front(Item {
            state: DeliveryState::Pending,
            ..item
        });
        return Err(error);
    }
    drop(queues);
    codetether_collaboration_state::execution_notify::signal(agent_id);
    Ok(())
}

/// Mark the matching front delivery pending again and persist it.
/// # Arguments
/// * `agent_id` — Durable child identity.
/// * `id` — Receipt expected at the queue front.
/// # Returns
/// Unit after persistence and completion notification.
/// # Errors
/// Returns missing/mismatched receipt or persistence errors.
/// # Examples
/// Implement [crate::Host] to provide the existing persistence and child runtime.
pub async fn release<H: Host>(agent_id: &str, id: &str) -> Result<()> {
    let mut queues = QUEUES.lock().await;
    let snapshot = queues
        .get_mut(agent_id)
        .ok_or_else(|| anyhow::anyhow!("mailbox missing"))?;
    let item = snapshot
        .items
        .front_mut()
        .filter(|item| item.id == id)
        .ok_or_else(|| anyhow::anyhow!("mailbox receipt mismatch"))?;
    item.state = DeliveryState::Pending;
    H::save(snapshot).await?;
    drop(queues);
    codetether_collaboration_state::execution_notify::signal(agent_id);
    Ok(())
}
