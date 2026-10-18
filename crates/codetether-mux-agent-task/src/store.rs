//! Bounded lookup and replay retention for mux agent tasks.

use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

use anyhow::{Result, bail};

use super::entry::AgentTask;

const TASK_LIMIT: usize = 128;

pub(super) struct TaskStore {
    pub(super) tasks: HashMap<String, Arc<AgentTask>>,
    pub(super) order: VecDeque<String>,
}

impl TaskStore {
    pub(super) fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            order: VecDeque::new(),
        }
    }

    /// Admit a new task for `session`; one turn runs per session at a time.
    pub(super) fn prepare(&mut self, task_id: &str, session: &str) -> Result<()> {
        if self.tasks.contains_key(task_id) {
            bail!("agent task {task_id} already exists");
        }
        if self
            .tasks
            .values()
            .any(|task| task.running() && task.session == session)
        {
            bail!("mux agent is already working");
        }
        while self.tasks.len() >= TASK_LIMIT {
            let Some(index) = self.order.iter().position(|id| !self.tasks[id].running()) else {
                bail!("mux agent task history is full");
            };
            if let Some(id) = self.order.remove(index) {
                self.tasks.remove(&id);
            }
        }
        Ok(())
    }
}
