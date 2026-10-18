//! Existing queue access operations, retained for wire and caller compatibility.

use super::{KnativeTask, KnativeTaskQueue};

impl KnativeTaskQueue {
    /// Append an incoming task.
    pub async fn push(&self, task: KnativeTask) {
        self.tasks.lock().await.push(task);
    }

    /// Remove the most recently appended task.
    pub async fn pop(&self) -> Option<KnativeTask> {
        self.tasks.lock().await.pop()
    }

    /// Return a snapshot of every task.
    pub async fn list(&self) -> Vec<KnativeTask> {
        self.tasks.lock().await.clone()
    }

    /// Return a snapshot of one task, if present.
    pub async fn get(&self, task_id: &str) -> Option<KnativeTask> {
        self.tasks
            .lock()
            .await
            .iter()
            .find(|task| task.task_id == task_id)
            .cloned()
    }

    /// Update an existing task, returning false when no task matches.
    pub async fn update_status(&self, task_id: &str, status: &str) -> bool {
        let mut tasks = self.tasks.lock().await;
        if let Some(task) = tasks.iter_mut().find(|task| task.task_id == task_id) {
            task.status = status.to_string();
            true
        } else {
            false
        }
    }
}
