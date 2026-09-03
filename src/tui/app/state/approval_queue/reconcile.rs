//! Removal of queue entries whose live waiter was resolved elsewhere.

use super::queue;

pub(crate) fn remove_stale() -> usize {
    let mut guard = queue().lock().expect("approval queue lock");
    let before = guard.len();
    guard.retain(|item| crate::approval::live::is_pending(&item.id));
    before.saturating_sub(guard.len())
}
