//! Durable append operations for child-agent mailboxes.

use super::{item::Item, snapshot::Snapshot, submission::Submission};
use crate::{EnqueueInput, Host};
use anyhow::Result;
use std::collections::HashMap;
use tokio::sync::Mutex;

lazy_static::lazy_static! {
    pub(super) static ref QUEUES: Mutex<HashMap<String, Snapshot>> = Mutex::new(HashMap::new());
}

/// Append one pending delivery and persist it before reporting acceptance.
/// # Arguments
/// * `agent_id` — Durable child identity.
/// * `message` — Queued input text.
/// * `params` — Borrowed image and parent metadata.
/// # Returns
/// The stable receipt and current queue depth.
/// # Errors
/// Returns snapshot load/write errors, rolling back the appended item on write failure.
/// # Examples
/// Implement [crate::Host] to provide the existing persistence and child runtime.
pub async fn enqueue<H: Host>(
    agent_id: &str,
    message: String,
    params: EnqueueInput<'_>,
) -> Result<Submission> {
    let mut queues = QUEUES.lock().await;
    ensure::<H>(&mut queues, agent_id).await?;
    let snapshot = queues.get_mut(agent_id).expect("mailbox inserted");
    let item = Item::new(
        message,
        params.images.to_vec(),
        params.parent_id.map(str::to_owned),
        params.prior_context,
    );
    let id = item.id.clone();
    snapshot.items.push_back(item);
    if let Err(error) = H::save(snapshot).await {
        snapshot.items.pop_back();
        return Err(error);
    }
    let len = snapshot.items.len();
    drop(queues);
    codetether_collaboration_state::execution_notify::signal(agent_id);
    Ok(Submission { id, depth: len })
}

pub(super) async fn ensure<H: Host>(
    queues: &mut HashMap<String, Snapshot>,
    agent_id: &str,
) -> Result<()> {
    if !queues.contains_key(agent_id) {
        queues.insert(agent_id.into(), H::load(agent_id).await?);
    }
    Ok(())
}
