use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use super::LiveApprovalDecision;

#[path = "state_entry.rs"]
mod entry;
pub(super) use entry::Pending;

#[derive(Default)]
struct State {
    latest: Option<String>,
    pending: HashMap<String, Pending>,
}

static STATE: OnceLock<Mutex<State>> = OnceLock::new();

fn state() -> &'static Mutex<State> {
    STATE.get_or_init(|| Mutex::new(State::default()))
}

pub(super) fn insert(id: String, pending: Pending) {
    let mut guard = state().lock().expect("approval state lock");
    guard.latest = Some(id.clone());
    guard.pending.insert(id, pending);
}

pub(super) fn remove(id: &str) {
    state()
        .lock()
        .expect("approval state lock")
        .pending
        .remove(id);
}

pub fn decide(id: &str, decision: LiveApprovalDecision) -> bool {
    let pending = state()
        .lock()
        .expect("approval state lock")
        .pending
        .remove(id);
    pending.is_some_and(|pending| pending.finish(id, decision))
}

pub fn latest_id() -> Option<String> {
    let guard = state().lock().expect("approval state lock");
    guard
        .latest
        .as_ref()
        .filter(|id| guard.pending.contains_key(*id))
        .cloned()
}
