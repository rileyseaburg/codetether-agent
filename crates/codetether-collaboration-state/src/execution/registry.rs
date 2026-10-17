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

/// Register a turn abort handle, or immediately cancel it if its agent is closed.
/// # Arguments
/// * `name` — Agent execution identity.
/// * `handle` — Spawned turn handle to register.
/// # Returns
/// Returns after registering or cancelling the handle.
/// # Panics
/// Panics if the execution registry lock is poisoned.
/// # Examples
/// ```no_run
/// # async fn example(){
/// let handle=tokio::spawn(std::future::pending::<()>());
/// codetether_collaboration_state::execution::register("docs-register",&handle);
/// handle.abort();
/// # }
/// ```
pub fn register<T>(name: &str, handle: &JoinHandle<T>) {
    let mut state = STATE.lock().expect("agent state lock poisoned");
    if state.closed.contains(name) {
        handle.abort();
    } else {
        state.aborts.insert(name.into(), handle.abort_handle());
    }
}

/// Abort the currently registered turn without removing its handle.
/// # Arguments
/// * `name` — Agent execution identity.
/// # Returns
/// True if a registered handle was found and aborted.
/// # Examples
/// ```
/// let _=codetether_collaboration_state::execution::abort("docs-register");
/// ```
pub fn abort(name: &str) -> bool {
    let handle = STATE
        .lock()
        .ok()
        .and_then(|state| state.aborts.get(name).cloned());
    handle.is_some_and(|handle| {
        handle.abort();
        true
    })
}

/// Check whether the agent currently holds an execution guard.
/// # Arguments
/// * `name` — Agent execution identity.
/// # Returns
/// True for a running entry; false if absent or the registry lock is poisoned.
/// # Examples
/// ```
/// let _=codetether_collaboration_state::execution::is_running("docs-agent");
/// ```
pub fn is_running(name: &str) -> bool {
    STATE.lock().is_ok_and(|state| state.running.contains(name))
}

/// Acquire the only execution guard for an open agent.
/// # Arguments
/// * `name` — Agent execution identity.
/// # Returns
/// A guard when the agent is open and idle; none if closed, running or the lock is poisoned.
/// # Examples
/// ```
/// let guard=codetether_collaboration_state::execution::try_start("docs-start");
/// drop(guard);
/// ```
pub fn try_start(name: &str) -> Option<AgentRunGuard> {
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
    crate::execution_notify::signal(name);
}
