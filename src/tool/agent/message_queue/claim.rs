//! Durable transition from pending to running delivery.

use super::{
    disk,
    item::{DeliveryState, Item},
    store::QUEUES,
};
use anyhow::Result;

pub(super) async fn next(agent_id: &str) -> Result<Option<Item>> {
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
    if let Err(error) = disk::save(snapshot).await {
        snapshot.items.front_mut().expect("claimed item").state = DeliveryState::Pending;
        return Err(error);
    }
    Ok(snapshot.items.front().cloned())
}
