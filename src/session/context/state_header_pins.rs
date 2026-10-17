//! Session views passed to shared pin-selection policy.
use crate::session::Session;
use codetether_context_policy::pins;
pub(super) fn pinned_indices(session: &Session) -> Vec<usize> {
    pins::pinned_indices(&session.pages, &session.messages)
}
pub(super) fn force_keep_pins(
    session: &Session,
    keep: &mut [bool],
    per_msg: &[usize],
    budget_for_messages: &mut usize,
) {
    pins::force_keep_pins(
        &session.pages,
        &session.messages,
        keep,
        per_msg,
        budget_for_messages,
    );
}
pub(super) fn force_keep_base(
    session: &Session,
    keep: &mut [bool],
    per_msg: &[usize],
    recent_start: usize,
    budget_for_messages: &mut usize,
) {
    pins::force_keep_base(
        &session.pages,
        &session.messages,
        keep,
        per_msg,
        recent_start,
        budget_for_messages,
    );
}
