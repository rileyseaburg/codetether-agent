//! Mailbox loading, inspection, and deletion.

use super::store::{QUEUES, ensure};
use crate::Host;
use anyhow::Result;

/// Count cached deliveries without loading a snapshot.
/// # Arguments
/// * `agent_id` — Durable child identity.
/// # Returns
/// The cached queue length, or zero if it has not been loaded.
/// # Examples
/// Implement [crate::Host] to provide the existing persistence and child runtime.
pub async fn pending(agent_id: &str) -> usize {
    QUEUES
        .lock()
        .await
        .get(agent_id)
        .map_or(0, |queue| queue.items.len())
}

/// Load a queue and persist crash-recovery state when necessary.
/// # Arguments
/// * `agent_id` — Durable child identity.
/// # Returns
/// Whether the loaded queue contains any deliveries.
/// # Errors
/// Returns snapshot load/recovery write errors.
/// # Examples
/// Implement [crate::Host] to provide the existing persistence and child runtime.
pub async fn hydrate<H: Host>(agent_id: &str) -> Result<bool> {
    let mut queues = QUEUES.lock().await;
    ensure::<H>(&mut queues, agent_id).await?;
    let snapshot = queues.get_mut(agent_id).expect("mailbox inserted");
    if snapshot.recover() {
        H::save(snapshot).await?;
    }
    Ok(!snapshot.items.is_empty())
}

/// Remove cached and persisted state for one child.
/// # Arguments
/// * `agent_id` — Durable child identity.
/// # Returns
/// Unit after persistence removal and completion notification.
/// # Errors
/// Returns persistence removal errors.
/// # Examples
/// Implement [crate::Host] to provide the existing persistence and child runtime.
pub async fn clear<H: Host>(agent_id: &str) -> Result<()> {
    QUEUES.lock().await.remove(agent_id);
    H::remove(agent_id).await?;
    codetether_collaboration_state::execution_notify::signal(agent_id);
    Ok(())
}

/// Transfer matching queued submissions to a new parent identity.
/// # Arguments
/// * `agent_id` — Durable child identity.
/// * `from` — Existing parent identity to match.
/// * `to` — Replacement parent identity.
/// # Returns
/// Unit after persisting changed submissions.
/// # Errors
/// Returns snapshot load/write errors; failed writes restore the original snapshot.
/// # Examples
/// Implement [crate::Host] to provide the existing persistence and child runtime.
pub async fn reparent_owner<H: Host>(agent_id: &str, from: &str, to: &str) -> Result<()> {
    let mut queues = QUEUES.lock().await;
    ensure::<H>(&mut queues, agent_id).await?;
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
        if let Err(error) = H::save(snapshot).await {
            *snapshot = original;
            return Err(error);
        }
    }
    Ok(())
}

/// Drop cached snapshots without changing persisted mailbox files.
/// Clear only the in-memory snapshot cache, preserving disk state.
/// # Arguments
/// This function has no arguments.
/// # Returns
/// Unit after dropping cached snapshots.
/// # Examples
/// Implement [crate::Host] to provide the existing persistence and child runtime.
pub async fn reset_cache() {
    QUEUES.lock().await.clear();
}
