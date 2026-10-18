//! Lifecycle owner for one direct `swarm_execute` invocation.

use super::finish;
use codetether_swarm_tool::task_result::TaskResult;
use codetether_swarm_view::SwarmEvent;
use std::time::Instant;
use tokio::sync::mpsc;

/// One invocation's event sender, start time and terminal-state ownership.
/// # Examples
/// ```
/// let mut observer = codetether_swarm_monitor::Observer::begin(&[], 20);
/// observer.complete(&[]);
/// ```
pub struct Observer {
    pub(super) events: mpsc::Sender<SwarmEvent>,
    pub(super) started_at: Instant,
    pub(super) terminal: bool,
}

impl Observer {
    /// Clone the shared event sender for a worker.
    pub fn sender(&self) -> mpsc::Sender<SwarmEvent> {
        self.events.clone()
    }

    /// Emit each worker result and the aggregate statistics, then mark terminal.
    pub fn complete(&mut self, results: &[TaskResult]) {
        finish::send(&self.events, results, self.started_at.elapsed());
        self.terminal = true;
    }

    /// Emit one invocation error and suppress later cancellation reporting.
    pub fn fail(&mut self, error: impl Into<String>) {
        let _ = self.events.try_send(SwarmEvent::Error(error.into()));
        self.terminal = true;
    }
}
