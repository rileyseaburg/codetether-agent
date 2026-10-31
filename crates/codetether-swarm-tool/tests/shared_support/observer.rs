//! Frontend event sink for the mocked direct swarm.
use codetether_swarm_core::SwarmEvent;
use codetether_swarm_tool::{runtime::Observer, task_input::TaskInput, task_result::TaskResult};
pub struct Sink(tokio::sync::mpsc::Sender<SwarmEvent>);
impl Observer for Sink {
    fn begin(_: &[TaskInput], _: usize) -> Self {
        let (tx, _) = tokio::sync::mpsc::channel(256);
        Self(tx)
    }
    fn sender(&self) -> tokio::sync::mpsc::Sender<SwarmEvent> {
        self.0.clone()
    }
    fn complete(&mut self, results: &[TaskResult]) {
        assert!(results.iter().all(|result| result.success));
    }
    fn fail(&mut self, error: impl Into<String>) {
        panic!("{}", error.into());
    }
}
