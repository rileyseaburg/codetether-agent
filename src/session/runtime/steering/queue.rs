//! Adapt typed session input and activity tracking to the shared runtime inboxes.

use std::sync::LazyLock;

use codetether_session_runtime::Inboxes;

use super::SteeringInput;

static INBOXES: LazyLock<Inboxes<SteeringInput>> = LazyLock::new(Inboxes::default);

/// Mark a session as accepting steering without discarding pending input.
pub(crate) fn open(session_id: &str) {
    INBOXES.open(session_id);
}

/// Append input when the named session is still accepting steering.
pub(crate) fn push(session_id: &str, input: SteeringInput) -> bool {
    let activity_id = input.activity_id();
    if INBOXES.push(session_id, input).is_err() {
        return false;
    }
    crate::tool::agent::collaboration_runtime::parent_activity::steered(session_id, activity_id);
    true
}

pub(super) fn discard(session_id: &str) -> Vec<SteeringInput> {
    INBOXES.discard(session_id)
}

/// Drain all inputs while leaving the named inbox open.
pub(super) fn take(session_id: &str) -> Vec<SteeringInput> {
    INBOXES.take(session_id)
}

/// Drain pending inputs or atomically close an empty inbox.
pub(super) fn take_or_close(session_id: &str) -> Vec<SteeringInput> {
    INBOXES.take_or_close(session_id)
}
