//! Synchronized FIFO storage keyed by session identifier.

use std::collections::{HashMap, VecDeque};
use std::sync::{LazyLock, Mutex};

use super::SteeringInput;

type Inboxes = HashMap<String, VecDeque<SteeringInput>>;

static INBOXES: LazyLock<Mutex<Inboxes>> = LazyLock::new(|| Mutex::new(HashMap::new()));

/// Mark a session as accepting steering without discarding pending input.
pub(crate) fn open(session_id: &str) {
    inboxes().entry(session_id.to_string()).or_default();
}

/// Append input when the named session is still accepting steering.
pub(crate) fn push(session_id: &str, input: SteeringInput) -> bool {
    let activity_id = input.activity_id();
    let accepted = {
        let mut inboxes = inboxes();
        let Some(inbox) = inboxes.get_mut(session_id) else {
            return false;
        };
        inbox.push_back(input);
        true
    };
    crate::tool::agent::collaboration_runtime::parent_activity::steered(session_id, activity_id);
    accepted
}

pub(super) fn discard(session_id: &str) -> VecDeque<SteeringInput> {
    inboxes().remove(session_id).unwrap_or_default()
}

/// Drain all inputs while leaving the named inbox open.
pub(super) fn take(session_id: &str) -> Vec<SteeringInput> {
    inboxes()
        .get_mut(session_id)
        .map(|inbox| inbox.drain(..).collect())
        .unwrap_or_default()
}

/// Drain pending inputs or atomically close an empty inbox.
pub(super) fn take_or_close(session_id: &str) -> Vec<SteeringInput> {
    let mut inboxes = inboxes();
    let Some(inbox) = inboxes.get_mut(session_id) else {
        return Vec::new();
    };
    if inbox.is_empty() {
        inboxes.remove(session_id);
        Vec::new()
    } else {
        inbox.drain(..).collect()
    }
}

fn inboxes() -> std::sync::MutexGuard<'static, Inboxes> {
    INBOXES.lock().expect("steering inbox mutex poisoned")
}
