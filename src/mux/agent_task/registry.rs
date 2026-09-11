//! Public task-registry operations used by the authenticated mux server.
//!
//! Every read and cancel is scoped to the requesting session; a task started
//! by one session is invisible to every other session on the same server.

use std::sync::{Arc, Mutex};

use anyhow::{Result, anyhow};

use super::{entry::AgentTask, store::TaskStore};

pub(in crate::mux) struct AgentTaskRegistry {
    pub(super) store: Mutex<TaskStore>,
}

impl AgentTaskRegistry {
    pub(in crate::mux) fn new() -> Self {
        Self {
            store: Mutex::new(TaskStore::new()),
        }
    }

    fn get(&self, session: &str, id: &str) -> Result<Arc<AgentTask>> {
        self.store
            .lock()
            .unwrap()
            .get(id)
            .filter(|task| task.session == session)
            .ok_or_else(|| anyhow!("agent task {id} was not found"))
    }

    pub(in crate::mux) async fn read(
        &self,
        session: &str,
        id: &str,
        offset: u64,
    ) -> Result<(Vec<u8>, u64, bool, Option<i32>)> {
        Ok(self.get(session, id)?.read(offset).await)
    }

    pub(in crate::mux) fn cancel(&self, session: &str, id: &str) -> Result<()> {
        let task = self.get(session, id)?;
        if task.running() {
            super::cancel::send(task.pid)?;
        }
        Ok(())
    }

    /// Interrupt every running turn owned by `session`.
    pub(in crate::mux) fn cancel_session(&self, session: &str) {
        for task in self.store.lock().unwrap().values() {
            if task.running() && task.session == session {
                let _ = super::cancel::send(task.pid);
            }
        }
    }

    pub(in crate::mux) fn cancel_all(&self) {
        for task in self.store.lock().unwrap().values() {
            if task.running() {
                let _ = super::cancel::send(task.pid);
            }
        }
    }
}
