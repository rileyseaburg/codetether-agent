//! Record completion and artifact events while the task guard is held.
use super::*;
pub(super) fn record(t: &Task, task_id: &String, artifact: Artifact) {
    // Emit streaming events for SSE consumers
    let status_event = TaskStatusUpdateEvent {
        id: task_id.clone(),
        status: t.status.clone(),
        is_final: true,
        metadata: std::collections::HashMap::new(),
    };
    let artifact_event = TaskArtifactUpdateEvent {
        id: task_id.clone(),
        artifact,
        metadata: std::collections::HashMap::new(),
    };
    tracing::debug!(target: "codetether_agent::a2a::server",
        task_id = %task_id,
        event = ?StreamEvent::StatusUpdate(status_event),
        "Task completed"
    );
    tracing::debug!(target: "codetether_agent::a2a::server",
        task_id = %task_id,
        event = ?StreamEvent::ArtifactUpdate(artifact_event),
        "Artifact produced"
    );
}
