//! Synchronized storage for exact session approval grants.

use crate::approval::ApprovalReceipt;
use std::collections::{HashMap, HashSet};
use std::sync::{Mutex, MutexGuard, OnceLock};

type Grant = (String, String, String, String);

#[derive(Default)]
struct State {
    requests: HashMap<String, String>,
    grants: HashSet<Grant>,
}

static STATE: OnceLock<Mutex<State>> = OnceLock::new();

fn state() -> MutexGuard<'static, State> {
    STATE
        .get_or_init(|| Mutex::new(State::default()))
        .lock()
        .expect("session approval grants lock")
}

pub(super) fn remember(id: &str, session_id: Option<&str>) {
    if let Some(session_id) = super::normalized_session(session_id) {
        state().requests.insert(id.into(), session_id.into());
    }
}

pub(super) fn grant(receipt: &ApprovalReceipt) {
    let mut state = state();
    if let Some(session) = state.requests.remove(&receipt.approval_id) {
        state.grants.insert((
            session,
            receipt.tool.clone(),
            receipt.action.clone(),
            receipt.resource.clone(),
        ));
    }
}

pub(super) fn discard(id: &str) {
    state().requests.remove(id);
}

pub(super) fn allowed(tool: &str, action: &str, resource: &str, session: Option<&str>) -> bool {
    let Some(session) = super::normalized_session(session) else {
        return false;
    };
    state().grants.contains(&(
        session.to_string(),
        tool.to_string(),
        action.to_string(),
        resource.to_string(),
    ))
}

#[cfg(test)]
pub(super) fn reset() {
    *state() = State::default();
}
