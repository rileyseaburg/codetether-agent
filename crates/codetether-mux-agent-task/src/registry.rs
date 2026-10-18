//! Public task-registry operations used by the authenticated mux server.
//!
//! Every read and cancel is scoped to the requesting session; a task started
//! by one session is invisible to every other session on the same server.

use std::sync::{Arc, Mutex};

use anyhow::{Result, anyhow};

use super::{entry::AgentTask, store::TaskStore};

/// Owns bounded replay and child lifecycles for session-scoped agent tasks.
///
/// # Examples
/// ```
/// let registry = codetether_mux_agent_task::AgentTaskRegistry::new();
/// registry.cancel_all();
/// ```
pub struct AgentTaskRegistry {
    pub(super) store: Mutex<TaskStore>,
}

impl AgentTaskRegistry {
    /// Construct an empty task registry.
    pub fn new() -> Self {
        Self {
            store: Mutex::new(TaskStore::new()),
        }
    }

    pub(super) fn get(&self, session: &str, id: &str) -> Result<Arc<AgentTask>> {
        self.store
            .lock()
            .unwrap()
            .get(id)
            .filter(|task| task.session == session)
            .ok_or_else(|| anyhow!("agent task {id} was not found"))
    }
}
