//! In-memory latest-snapshot state for recall workers.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use crate::session::Session;

struct State {
    pending: Option<Session>,
    running: bool,
}

pub(super) fn enqueue(session: &Session) -> bool {
    states().lock().is_ok_and(|mut states| {
        let state = states.entry(session.id.clone()).or_insert(State {
            pending: None,
            running: false,
        });
        state.pending = Some(session.clone());
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

pub(super) fn take(session_id: &str) -> Option<Session> {
    states().lock().ok()?.get_mut(session_id)?.pending.take()
}

pub(super) fn settle(session_id: &str) -> bool {
    let Ok(mut states) = states().lock() else {
        return true;
    };
    let Some(state) = states.get_mut(session_id) else {
        return true;
    };
    if state.pending.is_some() {
        return false;
    }
    state.running = false;
    true
}

fn states() -> &'static Mutex<HashMap<String, State>> {
    static STATES: OnceLock<Mutex<HashMap<String, State>>> = OnceLock::new();
    STATES.get_or_init(|| Mutex::new(HashMap::new()))
}
