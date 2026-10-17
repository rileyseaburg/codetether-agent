//! fail empty task operation.
use super::*;

pub fn fail_empty(tasks: &DashMap<String, Task>, task_id: &str) {
    if let Some(mut task) = tasks.get_mut(task_id) {
        task.status.state = TaskState::Failed;
        task.status.timestamp = Some(chrono::Utc::now().to_rfc3339());
    }
}
