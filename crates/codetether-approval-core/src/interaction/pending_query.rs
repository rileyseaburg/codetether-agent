//! Read-only queries over the live approval registry.

/// Return the newest registered request that is still awaiting a decision.
pub fn latest_id() -> Option<String> {
    let guard = super::pending::state().lock().expect("approval state lock");
    guard
        .order
        .iter()
        .rev()
        .find(|id| guard.pending.contains_key(*id))
        .cloned()
}

/// Return whether the registry still holds a reply sender for this request.
///
/// # Panics
/// Panics if the pending-request mutex was poisoned.
pub fn is_pending(id: &str) -> bool {
    super::pending::state()
        .lock()
        .expect("approval state lock")
        .pending
        .contains_key(id)
}
