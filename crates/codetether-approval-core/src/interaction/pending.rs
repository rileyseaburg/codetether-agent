//! Process-local pending approval registry and ordered delivery.

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

/// Register a reply sender and make its request the newest pending request.
///
/// # Panics
/// Panics if the pending-request mutex was poisoned.
pub fn insert(id: String, tx: Sender) {
    let mut guard = state().lock().expect("approval state lock");
    guard.order.retain(|pending| pending != &id);
    guard.order.push(id.clone());
    guard.pending.insert(id, tx);
}

/// Remove a pending reply sender and its ordering entry.
///
/// # Panics
/// Panics if the pending-request mutex was poisoned.
pub fn remove(id: &str) {
    let mut guard = state().lock().expect("approval state lock");
    guard.pending.remove(id);
    guard.order.retain(|pending| pending != id);
}

/// Remove a pending request and deliver its decision while holding the registry lock.
///
/// Returns whether a pending receiver accepted the decision.
/// # Panics
/// Panics if the pending-request mutex was poisoned.
pub fn decide(id: &str, decision: LiveApprovalDecision) -> bool {
    let mut guard = state().lock().expect("approval state lock");
    let sender = guard.pending.remove(id);
    guard.order.retain(|pending| pending != id);
    sender.is_some_and(|tx| tx.send(decision).is_ok())
}
