//! Durable acknowledgement and release of claimed mailbox items.

use super::{
    disk,
    item::{DeliveryState, Item},
    store::QUEUES,
};
use anyhow::{Result, bail};

pub(super) async fn ack(agent_id: &str, id: &str) -> Result<()> {
    let mut queues = QUEUES.lock().await;
    let snapshot = queues
        .get_mut(agent_id)
        .ok_or_else(|| anyhow::anyhow!("mailbox missing"))?;
    if snapshot.items.front().is_none_or(|item| item.id != id) {
        bail!("mailbox receipt mismatch");
    }
    let item = snapshot.items.pop_front().expect("receipt checked");
    if let Err(error) = disk::save(snapshot).await {
        snapshot.items.push_front(Item {
            state: DeliveryState::Pending,
            ..item
        });
        return Err(error);
    }
    drop(queues);
    super::super::execution_notify::signal(agent_id);
    Ok(())
}

pub(super) async fn release(agent_id: &str, id: &str) -> Result<()> {
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
    disk::save(snapshot).await?;
    drop(queues);
    super::super::execution_notify::signal(agent_id);
    Ok(())
}
