//! Removal of queue entries whose live waiter was resolved elsewhere.

use super::queue;

/// Remove requests for which the caller reports no live waiter.
/// # Arguments
/// * `is_pending` - Live-waiter predicate, called while the queue lock is held.
/// # Returns
/// The number of removed snapshots.
/// # Panics
/// Panics if the process-wide queue mutex was poisoned.
/// # Examples
/// ```no_run
/// let _ = codetether_approval_queue::remove_stale(|_| true);
/// ```
pub fn remove_stale(mut is_pending: impl FnMut(&str) -> bool) -> usize {
    let mut guard = queue().lock().expect("approval queue lock");
    let before = guard.len();
    guard.retain(|item| is_pending(&item.id));
    before.saturating_sub(guard.len())
}
