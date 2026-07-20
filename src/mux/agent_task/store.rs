//! Bounded lookup and replay retention for mux agent tasks.

use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

use anyhow::{Result, bail};

use super::entry::AgentTask;

const TASK_LIMIT: usize = 128;

pub(super) struct TaskStore {
    tasks: HashMap<String, Arc<AgentTask>>,
    order: VecDeque<String>,
}

impl TaskStore {
    pub(super) fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            order: VecDeque::new(),
        }
    }

    pub(super) fn prepare(&mut self, task_id: &str) -> Result<()> {
        if self.tasks.contains_key(task_id) {
            bail!("agent task {task_id} already exists");
        }
        if self.tasks.values().any(|task| task.running()) {
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

    pub(super) fn insert(&mut self, id: String, task: Arc<AgentTask>) {
        self.order.push_back(id.clone());
        self.tasks.insert(id, task);
    }

    pub(super) fn get(&self, id: &str) -> Option<Arc<AgentTask>> {
        self.tasks.get(id).cloned()
    }

    pub(super) fn values(&self) -> Vec<Arc<AgentTask>> {
        self.tasks.values().cloned().collect()
    }
}
