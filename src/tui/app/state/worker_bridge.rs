//! Worker bridge state operations.
//!
//! Methods for managing the A2A worker connection, registered agents,
//! and the incoming-task queue.

use crate::tui::worker_bridge::IncomingTask;

impl super::AppState {
    pub fn set_worker_bridge(&mut self, worker_id: String, worker_name: String) {
        self.worker.set_worker_bridge(worker_id, worker_name);
    }

    pub fn sync_worker_bridge_processing(&mut self, processing: bool) {
        self.worker.sync_worker_bridge_processing(processing);
    }

    pub fn register_worker_agent(&mut self, name: String) {
        self.worker.register_worker_agent(name);
    }

    pub fn push_recent_task(&mut self, task: String) {
        self.worker.push_recent_task(task);
    }

    pub fn enqueue_worker_task(&mut self, task: IncomingTask) {
        self.worker.enqueue_worker_task(task);
    }

    pub fn dequeue_worker_task(&mut self) -> Option<IncomingTask> {
        self.worker.dequeue_worker_task()
    }
}
