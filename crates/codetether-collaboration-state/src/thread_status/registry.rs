//! Watch channels keyed by canonical child session ID.

use super::ThreadStatus;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use tokio::sync::watch;

static STATUS: LazyLock<Mutex<HashMap<String, watch::Sender<ThreadStatus>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Create a pending status if the child has no channel.
/// # Arguments
/// * `agent_id` — Canonical child session ID.
/// # Returns
/// Returns after creating the channel or retaining the existing channel.
/// # Panics
/// Panics if the status registry lock is poisoned.
/// # Examples
/// ```
/// codetether_collaboration_state::thread_status::initialize("docs-child");
/// ```
pub fn initialize(agent_id: &str) {
    let mut status = STATUS.lock().expect("thread status lock poisoned");
    status.entry(agent_id.into()).or_insert_with(|| {
        let (tx, _) = watch::channel(ThreadStatus::PendingInit);
        tx
    });
}

/// Publish a new status and retain the channel for future subscribers.
/// # Arguments
/// * `agent_id` — Canonical child session ID.
/// * `next` — Status to publish and retain.
/// # Returns
/// Returns after replacing the current status and notifying subscribers.
/// # Panics
/// Panics if the status registry lock is poisoned.
/// # Examples
/// ```
/// codetether_collaboration_state::thread_status::set("docs-child",codetether_collaboration_state::ThreadStatus::Running);
/// ```
pub fn set(agent_id: &str, next: ThreadStatus) {
    let mut status = STATUS.lock().expect("thread status lock poisoned");
    let sender = status.entry(agent_id.into()).or_insert_with(|| {
        let (tx, _) = watch::channel(next.clone());
        tx
    });
    sender.send_replace(next);
}

/// Read current status, defaulting to NotFound when unavailable.
/// # Arguments
/// * `agent_id` — Canonical child session ID.
/// # Returns
/// The current status, or NotFound if the channel is missing or its registry is unavailable.
/// # Examples
/// ```
/// let _status=codetether_collaboration_state::thread_status::get("docs-child");
/// ```
pub fn get(agent_id: &str) -> ThreadStatus {
    STATUS
        .lock()
        .ok()
        .and_then(|status| status.get(agent_id).map(|tx| tx.borrow().clone()))
        .unwrap_or(ThreadStatus::NotFound)
}

/// Subscribe to an existing child status channel.
/// # Arguments
/// * `agent_id` — Canonical child session ID.
/// # Returns
/// A receiver for an existing channel, or none if the channel or registry is unavailable.
/// # Examples
/// ```
/// let _receiver=codetether_collaboration_state::thread_status::subscribe("docs-child");
/// ```
pub fn subscribe(agent_id: &str) -> Option<watch::Receiver<ThreadStatus>> {
    STATUS
        .lock()
        .ok()?
        .get(agent_id)
        .map(watch::Sender::subscribe)
}

/// Remove a child channel and publish NotFound to existing subscribers.
/// # Arguments
/// * `agent_id` — Canonical child session ID.
/// # Returns
/// Returns after removing an existing channel and publishing NotFound to its subscribers.
/// # Examples
/// ```
/// codetether_collaboration_state::thread_status::remove("docs-child");
/// ```
pub fn remove(agent_id: &str) {
    if let Ok(mut status) = STATUS.lock() {
        if let Some(sender) = status.remove(agent_id) {
            sender.send_replace(ThreadStatus::NotFound);
        }
    }
}
