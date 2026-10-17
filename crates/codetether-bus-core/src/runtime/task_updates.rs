//! task updates operations for the shared bus runtime.
use super::*;
impl BusHandle {
    /// Announce a task status update.
    pub fn send_task_update(
        &self,
        task_id: &str,
        state: TaskState,
        message: Option<String>,
    ) -> usize {
        self.send(
            format!("task.{task_id}"),
            BusMessage::TaskUpdate {
                task_id: task_id.to_string(),
                state,
                message,
            },
        )
    }

    /// Announce an artifact update.
    pub fn send_artifact_update(&self, task_id: &str, artifact: Artifact) -> usize {
        self.send(
            format!("task.{task_id}"),
            BusMessage::ArtifactUpdate {
                task_id: task_id.to_string(),
                artifact,
            },
        )
    }
}
