//! Reconcile queued approval displays through the live application predicate.
/// Declare the existing stale-removal helper using an application predicate.
/// # Usage
/// ```text
/// codetether_approval_queue::define_reconciler!(is_pending);
/// ```
#[macro_export]
macro_rules! define_reconciler {
    ($pending:path) => {
        pub(crate) fn remove_stale() -> usize {
            $crate::remove_stale($pending)
        }
    };
}
