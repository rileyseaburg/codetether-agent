use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use tokio::sync::oneshot;

use super::LiveApprovalDecision;

type Sender = oneshot::Sender<LiveApprovalDecision>;

#[derive(Default)]
pub(super) struct State {
    pub(super) order: Vec<String>,
    pub(super) pending: HashMap<String, Sender>,
}

static STATE: OnceLock<Mutex<State>> = OnceLock::new();

pub(super) fn state() -> &'static Mutex<State> {
    STATE.get_or_init(|| Mutex::new(State::default()))
}

pub(super) fn insert(id: String, tx: Sender) {
    let mut guard = state().lock().expect("approval state lock");
    guard.order.retain(|pending| pending != &id);
    guard.order.push(id.clone());
    guard.pending.insert(id, tx);
}

pub(super) fn remove(id: &str) {
    let mut guard = state().lock().expect("approval state lock");
    guard.pending.remove(id);
    guard.order.retain(|pending| pending != id);
}

pub fn decide(id: &str, decision: LiveApprovalDecision) -> bool {
    let mut guard = state().lock().expect("approval state lock");
    let sender = guard.pending.remove(id);
    guard.order.retain(|pending| pending != id);
    sender.is_some_and(|tx| tx.send(decision).is_ok())
}
