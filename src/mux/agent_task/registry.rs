//! Public task-registry operations used by the authenticated mux server.

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

    fn get(&self, id: &str) -> Result<Arc<AgentTask>> {
        self.store
            .lock()
            .unwrap()
            .get(id)
            .ok_or_else(|| anyhow!("agent task {id} was not found"))
    }

    pub(in crate::mux) async fn read(
        &self,
        id: &str,
        offset: u64,
    ) -> Result<(Vec<u8>, u64, bool, Option<i32>)> {
        Ok(self.get(id)?.read(offset).await)
    }

    pub(in crate::mux) fn cancel(&self, id: &str) -> Result<()> {
        let task = self.get(id)?;
        if task.running() {
            super::cancel::send(task.pid)?;
        }
        Ok(())
    }

    pub(in crate::mux) fn cancel_all(&self) {
        let tasks = self.store.lock().unwrap().values();
        for task in tasks {
            if task.running() {
                let _ = super::cancel::send(task.pid);
            }
        }
    }
}
