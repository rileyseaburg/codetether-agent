//! Task spawning support.

use crate::scheduler::queue::Scheduler;
use crate::scheduler::task::Task;

impl Scheduler {
    /// Create a new ready task and return its id.
    pub fn spawn(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id = self.next_id.saturating_add(1);
        self.tasks.insert(id, Task::ready(id));
        self.ready.push_back(id);
        id
    }
}
