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

pub(crate) async fn reparent_owner(agent_id: &str, from: &str, to: &str) -> Result<()> {
    let mut queues = QUEUES.lock().await;
    ensure(&mut queues, agent_id).await?;
    let snapshot = queues.get_mut(agent_id).expect("mailbox inserted");
    let original = snapshot.clone();
    let mut changed = false;
    for item in &mut snapshot.items {
        if item.parent_id.as_deref() == Some(from) {
            item.parent_id = Some(to.to_string());
            changed = true;
        }
    }
    if changed {
        if let Err(error) = disk::save(snapshot).await {
            *snapshot = original;
            return Err(error);
        }
    }
    Ok(())
}
