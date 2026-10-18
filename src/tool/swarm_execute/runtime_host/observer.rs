//! Preserve native monitor events and cancellation ownership.
use super::super::tui_bridge;
use codetether_swarm_tool::{runtime::Observer, task_input::TaskInput, task_result::TaskResult};
impl Observer for tui_bridge::Observer {
    fn begin(tasks: &[TaskInput], max_steps: usize) -> Self {
        Self::begin(tasks, max_steps)
    }
    fn sender(&self) -> tokio::sync::mpsc::Sender<codetether_swarm_core::SwarmEvent> {
        self.sender()
    }
    fn complete(&mut self, results: &[TaskResult]) {
        self.complete(results);
    }
}
