//! Read-only queries over the live approval registry.

/// Return the newest registered request that is still awaiting a decision.
pub fn latest_id() -> Option<String> {
    let guard = super::state::state().lock().expect("approval state lock");
    guard
        .order
        .iter()
        .rev()
        .find(|id| guard.pending.contains_key(*id))
        .cloned()
}

pub(crate) fn is_pending(id: &str) -> bool {
    super::state::state()
        .lock()
        .expect("approval state lock")
        .pending
        .contains_key(id)
}
