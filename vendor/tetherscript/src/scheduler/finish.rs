//! Task completion and waiter wake-up support.

use crate::scheduler::queue::Scheduler;
use crate::scheduler::task::TaskState;
use crate::value::Value;

impl Scheduler {
    /// Mark a task done, store its result, and wake satisfied waiters.
    pub fn finish(&mut self, id: u64, result: Value) {
        if let Some(task) = self.tasks.get_mut(&id) {
            task.state = TaskState::Done;
            task.result = Some(result);
        }
        let waiters = self
            .tasks
            .get_mut(&id)
            .map(|t| std::mem::take(&mut t.waiters))
            .unwrap_or_default();
        for waiter in waiters {
            self.try_wake(waiter);
        }
    }

    pub(super) fn try_wake(&mut self, id: u64) {
        if !self.waiting_deps_done(id) {
            return;
        }
        if let Some(task) = self.tasks.get_mut(&id) {
            task.state = TaskState::Ready;
            task.waiting_for.clear();
        }
        self.ready.push_back(id);
    }
}
