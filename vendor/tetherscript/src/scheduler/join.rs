//! Join dependency tracking for cooperative tasks.

use crate::scheduler::queue::Scheduler;
use crate::scheduler::task::TaskState;
use crate::value::Value;

impl Scheduler {
    /// Wait for target task ids or return their completed values.
    pub fn join(&mut self, waiter: u64, targets: &[u64]) -> Option<Vec<Value>> {
        if self.targets_done(targets) {
            return Some(self.values(targets));
        }
        if let Some(task) = self.tasks.get_mut(&waiter) {
            task.state = TaskState::Waiting;
            task.waiting_for = targets.to_vec();
        }
        for target in targets {
            if let Some(task) = self.tasks.get_mut(target) {
                task.waiters.push(waiter);
            }
        }
        None
    }

    fn targets_done(&self, targets: &[u64]) -> bool {
        targets
            .iter()
            .all(|id| matches!(self.tasks.get(id).map(|t| t.state), Some(TaskState::Done)))
    }

    fn values(&self, targets: &[u64]) -> Vec<Value> {
        targets
            .iter()
            .filter_map(|id| self.tasks.get(id).and_then(|t| t.result.clone()))
            .collect()
    }

    pub(super) fn waiting_deps_done(&self, id: u64) -> bool {
        self.tasks
            .get(&id)
            .map(|t| self.targets_done(&t.waiting_for))
            .unwrap_or(false)
    }
}
