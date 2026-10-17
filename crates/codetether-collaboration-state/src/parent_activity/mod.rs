//! Lossless parent wake-ups for child results and steered user input.

#[path = "channel.rs"]
mod channel;
#[path = "registry.rs"]
mod registry;
#[path = "wait.rs"]
mod wait;

mod activity;
pub use activity::Activity;

/// Publish a child mailbox update for its parent.
/// # Arguments
/// * `owner` — Parent session whose queue receives the notification.
/// # Returns
/// Returns after queueing the event and waking waiters.
/// # Panics
/// Panics if an internal registry or channel lock was poisoned.
/// # Examples
/// ```
/// codetether_collaboration_state::parent_activity::mailbox("docs-mailbox");
/// ```
pub fn mailbox(owner: &str) {
    registry::publish(owner, Activity::Mailbox);
}

/// Publish a parent input notification.
/// # Arguments
/// * `owner` — Parent session whose queue receives the notification.
/// * `input_id` — Input identity used for later acknowledgement.
/// # Returns
/// Returns after queueing the event and waking waiters.
/// # Panics
/// Panics if an internal registry or channel lock was poisoned.
/// # Examples
/// ```
/// codetether_collaboration_state::parent_activity::steered("docs-steered",1);
/// ```
pub fn steered(owner: &str, input_id: u64) {
    registry::publish(owner, Activity::Steered(input_id));
}

/// Remove a matching input notification after the parent accepts that input.
/// # Arguments
/// * `owner` — Parent session whose queue is updated.
/// * `input_id` — Input identity to acknowledge.
/// # Returns
/// Returns after removing the first matching pending notification, if present.
/// # Panics
/// Panics if an internal registry or channel lock was poisoned.
/// # Examples
/// ```
/// codetether_collaboration_state::parent_activity::acknowledge_steered("docs-steered",1);
/// ```
pub fn acknowledge_steered(owner: &str, input_id: u64) {
    registry::remove(owner, Activity::Steered(input_id));
}

/// Consume pending activity or wait for new activity until a deadline.
/// # Arguments
/// * `owner` — Parent session to observe.
/// * `deadline` — Tokio instant at which waiting ends.
/// # Returns
/// The oldest queued activity, or none when the deadline expires.
/// # Panics
/// Panics if an internal registry or channel lock was poisoned.
/// # Examples
/// ```no_run
/// # async fn example(){
/// let result=codetether_collaboration_state::parent_activity::until("docs-wait",tokio::time::Instant::now()).await;
/// assert!(result.is_none());
/// # }
/// ```
pub async fn until(owner: &str, deadline: tokio::time::Instant) -> Option<Activity> {
    wait::until(registry::channel(owner), deadline).await
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
