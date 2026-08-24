//! Cleanup for abandoned live approval waiters.

pub(super) fn cancel(id: &str) {
    let undecided = crate::approval::ApprovalStore::open_default()
        .and_then(|store| store.decision(id))
        .is_ok_and(|decision| decision.is_none());
    if undecided && let Ok(store) = crate::approval::ApprovalStore::open_default() {
        let _ = store.deny(id, "runtime", "approval waiter cancelled");
    }
    crate::approval::session_settle::request(id, None);
}
