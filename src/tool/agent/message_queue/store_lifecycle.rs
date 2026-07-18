//! Mailbox loading, inspection, and deletion.

use super::{
    disk,
    store::{QUEUES, ensure},
};
use anyhow::Result;

pub(crate) async fn pending(agent_id: &str) -> usize {
    QUEUES
        .lock()
        .await
        .get(agent_id)
        .map_or(0, |queue| queue.items.len())
}

pub(crate) async fn hydrate(agent_id: &str) -> Result<bool> {
    let mut queues = QUEUES.lock().await;
    ensure(&mut queues, agent_id).await?;
    let snapshot = queues.get_mut(agent_id).expect("mailbox inserted");
    if snapshot.recover() {
        disk::save(snapshot).await?;
    }
    Ok(!snapshot.items.is_empty())
}

pub(crate) async fn clear(agent_id: &str) -> Result<()> {
    QUEUES.lock().await.remove(agent_id);
    disk::remove(agent_id).await?;
    super::super::execution_notify::signal(agent_id);
    Ok(())
}
