//! Completion notifications for child-agent state transitions.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::Notify;

lazy_static::lazy_static! {
    static ref NOTIFIERS: Mutex<HashMap<String, Arc<Notify>>> = Mutex::new(HashMap::new());
}

/// Return the shared completion notifier for an agent.
/// # Arguments
/// * `name` — Agent execution identity.
/// # Returns
/// The existing notifier or a new notifier registered for this identity.
/// # Panics
/// Panics if the notifier registry lock is poisoned.
/// # Examples
/// ```
/// let _=codetether_collaboration_state::execution_notify::notification("docs-notify");
/// ```
pub fn notification(name: &str) -> Arc<Notify> {
    NOTIFIERS
        .lock()
        .expect("agent notifier lock poisoned")
        .entry(name.to_string())
        .or_insert_with(|| Arc::new(Notify::new()))
        .clone()
}

/// Wake current waiters and retain a permit for a future waiter.
/// # Arguments
/// * `name` — Agent execution identity.
/// # Returns
/// Returns after notifying both current and subsequent waiters.
/// # Panics
/// Panics if the notifier registry lock is poisoned.
/// # Examples
/// ```
/// codetether_collaboration_state::execution_notify::signal("docs-notify");
/// ```
pub fn signal(name: &str) {
    let notify = notification(name);
    notify.notify_waiters();
    notify.notify_one();
}
