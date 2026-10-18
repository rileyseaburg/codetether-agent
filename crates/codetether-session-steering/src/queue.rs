//! Adapt typed session input and activity tracking to the shared runtime inboxes.

use std::sync::LazyLock;

use codetether_session_runtime::Inboxes;

use super::SteeringInput;

static INBOXES: LazyLock<Inboxes<SteeringInput>> = LazyLock::new(Inboxes::default);

/// Mark a session as accepting steering without discarding pending input.
/// # Arguments
/// The durable identity of the session accepting steering.
/// # Returns
/// Unit; already queued messages are preserved.
/// # Examples
/// ```rust
/// codetether_session_steering::open("session-1234");
/// ```
pub fn open(session_id: &str) {
    INBOXES.open(session_id);
}

/// Append input when the named session is still accepting steering.
/// # Arguments
/// The session ID and input to enqueue.
/// # Returns
/// Whether the currently open inbox accepted the input.
/// # Examples
/// ```rust
/// use codetether_session_steering::{open, push, clear, SteeringInput};
/// open("session-1234");
/// assert!(push("session-1234", SteeringInput::new("Continue".into(), vec![])));
/// clear("session-1234");
/// ```
pub fn push(session_id: &str, input: SteeringInput) -> bool {
    let activity_id = input.activity_id();
    if INBOXES.push(session_id, input).is_err() {
        return false;
    }
    codetether_collaboration_state::parent_activity::steered(session_id, activity_id);
    true
}

pub(crate) fn discard(session_id: &str) -> Vec<SteeringInput> {
    INBOXES.discard(session_id)
}

/// Drain all inputs while leaving the named inbox open.
pub(crate) fn take(session_id: &str) -> Vec<SteeringInput> {
    INBOXES.take(session_id)
}

/// Drain pending inputs or atomically close an empty inbox.
pub(crate) fn take_or_close(session_id: &str) -> Vec<SteeringInput> {
    INBOXES.take_or_close(session_id)
}
