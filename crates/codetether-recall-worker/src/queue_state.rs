//! Enqueue, remove and take the latest pending session ID.
use super::state::{State, states};

pub(super) fn enqueue(session_id: &str) -> bool {
    states().lock().is_ok_and(|mut states| {
        let state = states.entry(session_id.to_string()).or_insert(State {
            pending: None,
            running: false,
        });
        state.pending = Some(session_id.to_string());
        let spawn = !state.running;
        state.running = true;
        spawn
    })
}

pub(super) fn remove(session_id: &str) {
    if let Ok(mut states) = states().lock() {
        states.remove(session_id);
    }
}

pub(super) fn take(session_id: &str) -> Option<String> {
    states().lock().ok()?.get_mut(session_id)?.pending.take()
}
