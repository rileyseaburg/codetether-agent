//! Reconcile display snapshots with the native live approval waiters.
pub(crate) fn remove_stale() -> usize {
    codetether_approval_queue::remove_stale(crate::approval::live::is_pending)
}
