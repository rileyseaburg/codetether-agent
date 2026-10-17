//! Durable transition from pending to running delivery.

use super::{
    item::{DeliveryState, Item},
    store::QUEUES,
};
use crate::Host;
use anyhow::Result;

/// Claim the pending front delivery and persist its running state.
/// # Arguments
/// * `agent_id` — Durable child identity.
/// # Returns
/// A cloned front item, or none when absent or already running.
/// # Errors
/// Returns persistence errors after reverting the claimed item to pending.
/// # Examples
/// Implement [crate::Host] to provide the existing persistence and child runtime.
pub async fn next<H: Host>(agent_id: &str) -> Result<Option<Item>> {
    let mut queues = QUEUES.lock().await;
    let Some(snapshot) = queues.get_mut(agent_id) else {
        return Ok(None);
    };
    let Some(item) = snapshot.items.front_mut() else {
        return Ok(None);
    };
    if item.state == DeliveryState::Running {
        return Ok(None);
    }
    item.state = DeliveryState::Running;
    if let Err(error) = H::save(snapshot).await {
        snapshot.items.front_mut().expect("claimed item").state = DeliveryState::Pending;
        return Err(error);
    }
    Ok(snapshot.items.front().cloned())
}
