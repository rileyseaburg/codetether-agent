//! Bounded incoming task queue.
//! Hosts use WorkerState independently of the bridge transport lifecycle.

use crate::IncomingTask;
impl crate::WorkerState {
    /// Enqueue a task, dropping oldest entries beyond twenty.
    ///
    /// # Arguments
    /// * `task` — Incoming task to enqueue.
    ///
    /// # Returns
    /// Updates the worker state in place.
    ///
    /// # Examples
    /// ```
    /// let mut state = codetether_tui_worker::WorkerState::default();
    /// state.enqueue_worker_task(codetether_tui_worker::IncomingTask { task_id: "id".into(), message: "review".into(), from_agent: None });
    /// assert_eq!(state.worker_task_queue.len(), 1);
    /// ```

    pub fn enqueue_worker_task(&mut self, task: IncomingTask) {
        self.worker_task_queue.push_back(task);
        while self.worker_task_queue.len() > 20 {
            self.worker_task_queue.pop_front();
        }
    }
    /// Remove the oldest queued task.
    ///
    /// # Returns
    /// The oldest task, or None when the queue is empty.
    ///
    /// # Examples
    /// ```
    /// let mut state = codetether_tui_worker::WorkerState::default();
    /// assert!(state.dequeue_worker_task().is_none());
    /// ```

    pub fn dequeue_worker_task(&mut self) -> Option<IncomingTask> {
        self.worker_task_queue.pop_front()
    }
}
