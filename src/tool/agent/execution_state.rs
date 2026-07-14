//! Per-agent execution state for spawned child sessions.

use std::{
    collections::{HashMap, HashSet},
    sync::Mutex,
};
use tokio::task::{AbortHandle, JoinHandle};

lazy_static::lazy_static! {
    static ref RUNNING: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
    static ref ABORTS: Mutex<HashMap<String, AbortHandle>> = Mutex::new(HashMap::new());
}

/// Guard representing the active message run for one sub-agent.
pub(super) struct AgentRunGuard {
    name: String,
}

impl Drop for AgentRunGuard {
    fn drop(&mut self) {
        if let Ok(mut running) = RUNNING.lock() {
            running.remove(&self.name);
        }
        if let Ok(mut aborts) = ABORTS.lock() {
            aborts.remove(&self.name);
        }
    }
}

pub(super) fn register<T>(name: &str, handle: &JoinHandle<T>) {
    if let Ok(mut aborts) = ABORTS.lock() {
        aborts.insert(name.to_string(), handle.abort_handle());
    }
}

pub(super) fn abort(name: &str) -> bool {
    let handle = ABORTS
        .lock()
        .ok()
        .and_then(|mut aborts| aborts.remove(name));
    if let Some(handle) = handle {
        handle.abort();
        true
    } else {
        false
    }
}

/// Tries to mark a sub-agent as busy.
pub(super) fn try_start(name: &str) -> Option<AgentRunGuard> {
    let mut running = RUNNING.lock().ok()?;
    if !running.insert(name.to_string()) {
        return None;
    }
    Some(AgentRunGuard {
        name: name.to_string(),
    })
}
