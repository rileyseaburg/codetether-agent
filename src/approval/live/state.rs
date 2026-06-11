use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use tokio::sync::oneshot;

use super::LiveApprovalDecision;

type Sender = oneshot::Sender<LiveApprovalDecision>;

#[derive(Default)]
struct State {
    latest: Option<String>,
    pending: HashMap<String, Sender>,
}

static STATE: OnceLock<Mutex<State>> = OnceLock::new();

fn state() -> &'static Mutex<State> {
    STATE.get_or_init(|| Mutex::new(State::default()))
}

pub(super) fn insert(id: String, tx: Sender) {
    let mut guard = state().lock().expect("approval state lock");
    guard.latest = Some(id.clone());
    guard.pending.insert(id, tx);
}

pub(super) fn remove(id: &str) {
    state()
        .lock()
        .expect("approval state lock")
        .pending
        .remove(id);
}

pub fn decide(id: &str, decision: LiveApprovalDecision) -> bool {
    let sender = state()
        .lock()
        .expect("approval state lock")
        .pending
        .remove(id);
    sender.is_some_and(|tx| tx.send(decision).is_ok())
}

pub fn latest_id() -> Option<String> {
    let guard = state().lock().expect("approval state lock");
    guard
        .latest
        .as_ref()
        .filter(|id| guard.pending.contains_key(*id))
        .cloned()
}
