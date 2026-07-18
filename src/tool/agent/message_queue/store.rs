//! Durable append operations for child-agent mailboxes.

use super::{disk, item::Item, snapshot::Snapshot, submission::Submission};
use crate::tool::agent::params::Params;
use anyhow::Result;
use std::collections::HashMap;
use tokio::sync::Mutex;

lazy_static::lazy_static! {
    pub(super) static ref QUEUES: Mutex<HashMap<String, Snapshot>> = Mutex::new(HashMap::new());
}

pub(super) async fn enqueue(
    agent_id: &str,
    message: String,
    params: &Params,
) -> Result<Submission> {
    let mut queues = QUEUES.lock().await;
    ensure(&mut queues, agent_id).await?;
    let snapshot = queues.get_mut(agent_id).expect("mailbox inserted");
    let item = Item::new(
        message,
        params.message_images.clone(),
        params.parent_session_id.clone(),
        params.parent_prior_context_allowed,
    );
    let id = item.id.clone();
    snapshot.items.push_back(item);
    if let Err(error) = disk::save(snapshot).await {
        snapshot.items.pop_back();
        return Err(error);
    }
    let len = snapshot.items.len();
    drop(queues);
    super::super::execution_notify::signal(agent_id);
    Ok(Submission { id, depth: len })
}

pub(super) async fn ensure(queues: &mut HashMap<String, Snapshot>, agent_id: &str) -> Result<()> {
    if !queues.contains_key(agent_id) {
        queues.insert(agent_id.into(), disk::load(agent_id).await?);
    }
    Ok(())
}
