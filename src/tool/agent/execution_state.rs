//! Per-agent execution state for spawned child sessions.

use std::{collections::HashSet, sync::Mutex};

lazy_static::lazy_static! {
    static ref RUNNING: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
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
