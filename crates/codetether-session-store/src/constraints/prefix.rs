//! Expose full checkpoint source records to request derivation.
/// Return the snapshot's pinned sources with absolute turn numbers.
/// # Arguments
/// `snapshot` supplies the instance-local checkpoint and its retained excerpts.
/// # Returns
/// Complete source records in checkpoint order.
/// # Panics
/// Panics if the checkpoint mutex is poisoned, matching native session behavior.
/// # Examples
/// ```no_run
/// # fn example(snapshot: &impl codetether_session_store::Snapshot) {
/// let _lines = codetether_session_store::constraints::prefix(snapshot);
/// # }
/// ```
pub fn prefix(snapshot: &impl crate::Snapshot) -> Vec<super::ConstraintRecord> {
    snapshot.state().0.lock().unwrap().constraints.clone()
}
