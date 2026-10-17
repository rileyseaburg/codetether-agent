//! proto task to local conversion.
use super::*;

/// Convert a proto `Task` to a local `Task`.
pub fn proto_task_to_local(task: &proto::Task) -> local::Task {
    let status = task
        .status
        .as_ref()
        .map(|s| local::TaskStatus {
            state: proto_task_state_to_local(s.state()),
            message: s.update.as_ref().map(proto_message_to_local),
            timestamp: s.timestamp.as_ref().map(|t| {
                chrono::DateTime::from_timestamp(t.seconds, t.nanos as u32)
                    .map(|dt| dt.to_rfc3339())
                    .unwrap_or_default()
            }),
        })
        .unwrap_or(local::TaskStatus {
            state: local::TaskState::Submitted,
            message: None,
            timestamp: None,
        });

    local::Task {
        id: task.id.clone(),
        context_id: if task.context_id.is_empty() {
            None
        } else {
            Some(task.context_id.clone())
        },
        status,
        artifacts: task.artifacts.iter().map(proto_artifact_to_local).collect(),
        history: task.history.iter().map(proto_message_to_local).collect(),
        metadata: Default::default(),
    }
}
