//! Atomic registry of running, aborted, and closed child agents.

use super::guard::AgentRunGuard;
use std::collections::{HashMap, HashSet};
use std::sync::{LazyLock, Mutex};
use tokio::task::{AbortHandle, JoinHandle};

#[derive(Default)]
pub(super) struct State {
    pub(super) running: HashSet<String>,
    pub(super) aborts: HashMap<String, AbortHandle>,
    pub(super) closed: HashSet<String>,
}

pub(super) static STATE: LazyLock<Mutex<State>> = LazyLock::new(|| Mutex::new(State::default()));

pub(in crate::tool::agent) fn register<T>(name: &str, handle: &JoinHandle<T>) {
    let mut state = STATE.lock().expect("agent state lock poisoned");
    if state.closed.contains(name) {
        handle.abort();
    } else {
        state.aborts.insert(name.into(), handle.abort_handle());
    }
}

pub(in crate::tool::agent) fn abort(name: &str) -> bool {
    let handle = STATE
        .lock()
        .ok()
        .and_then(|state| state.aborts.get(name).cloned());
    handle.is_some_and(|handle| {
        handle.abort();
        true
    })
}

pub(in crate::tool::agent) fn is_running(name: &str) -> bool {
    STATE.lock().is_ok_and(|state| state.running.contains(name))
}

pub(in crate::tool::agent) fn try_start(name: &str) -> Option<AgentRunGuard> {
    let mut state = STATE.lock().ok()?;
    if state.closed.contains(name) || !state.running.insert(name.into()) {
        return None;
    }
    Some(AgentRunGuard { name: name.into() })
}

pub(super) fn finish(name: &str) {
    if let Ok(mut state) = STATE.lock() {
        state.running.remove(name);
        state.aborts.remove(name);
    }
    super::super::collaboration_runtime::execution_notify::signal(name);
}
