//! Record session-construction failure before returning.
use super::*;
pub(super) fn record(
    tasks: &DashMap<String, Task>,
    task_id: &str,
    prompt: &str,
    started_at: Instant,
    e: anyhow::Error,
) {
    tracing::error!(target: "codetether_agent::a2a::server",
        "Failed to create session for stream task {}: {}",
        task_id,
        e
    );
    if let Some(mut t) = tasks.get_mut(task_id) {
        t.status.state = TaskState::Failed;
        t.status.timestamp = Some(chrono::Utc::now().to_rfc3339());
    }
    record_a2a_message_telemetry(
        "a2a_message_stream",
        &task_id,
        false,
        &prompt,
        started_at.elapsed(),
        false,
        None,
        Some(e.to_string()),
    );
}
