//! Worker bridge state operations.
//!
//! Methods for managing the A2A worker connection, registered agents,
//! and the incoming-task queue.

use crate::tui::worker_bridge::IncomingTask;

impl super::AppState {
    pub fn set_worker_bridge(&mut self, worker_id: String, worker_name: String) {
        self.worker_id = Some(worker_id);
        self.worker_name = Some(worker_name);
        self.a2a_connected = true;
    }

    pub fn sync_worker_bridge_processing(&mut self, processing: bool) {
        self.worker_bridge_processing_state = Some(processing);
    }

    pub fn register_worker_agent(&mut self, name: String) {
        self.worker_bridge_registered_agents.insert(name);
    }

    pub fn push_recent_task(&mut self, task: String) {
        self.recent_tasks.push(task);
        if self.recent_tasks.len() > 20 {
            let drain_len = self.recent_tasks.len() - 20;
            self.recent_tasks.drain(0..drain_len);
        }
    }

    pub fn enqueue_worker_task(&mut self, task: IncomingTask) {
        self.worker_task_queue.push_back(task);
        while self.worker_task_queue.len() > 20 {
            self.worker_task_queue.pop_front();
        }
    }

    pub fn dequeue_worker_task(&mut self) -> Option<IncomingTask> {
        self.worker_task_queue.pop_front()
    }
}
