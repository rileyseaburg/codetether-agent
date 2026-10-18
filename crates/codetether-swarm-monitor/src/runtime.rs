//! Preserve native monitor events and cancellation ownership.
use crate::Observer;
use codetether_swarm_tool::{
    runtime::Observer as RuntimeObserver, task_input::TaskInput, task_result::TaskResult,
};
impl RuntimeObserver for Observer {
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
