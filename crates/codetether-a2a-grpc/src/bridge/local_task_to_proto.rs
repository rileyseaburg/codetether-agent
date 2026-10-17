//! local task to proto conversion.
use super::*;

// ═══════════════════════════════════════════════════════════════════════
// Local → Proto
// ═══════════════════════════════════════════════════════════════════════

/// Convert a local `Task` to a proto `Task`.
pub fn local_task_to_proto(task: &local::Task) -> proto::Task {
    proto::Task {
        id: task.id.clone(),
        context_id: task.context_id.clone().unwrap_or_default(),
        status: Some(local_task_status_to_proto(&task.status)),
        artifacts: task.artifacts.iter().map(local_artifact_to_proto).collect(),
        history: task.history.iter().map(local_message_to_proto).collect(),
        metadata: None,
    }
}
