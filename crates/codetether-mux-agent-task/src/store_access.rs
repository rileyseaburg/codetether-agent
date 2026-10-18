//! Lookup, insertion and iteration for retained task records.
use super::{entry::AgentTask, store::TaskStore};
use std::sync::Arc;

impl TaskStore {
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
