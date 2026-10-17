//! Record a failed stream execution.
use super::*;
/// Record a failed stream execution and its telemetry.
///
/// # Arguments
///
/// Supply the task table, task id, prompt, start time and execution error.
///
/// # Returns
///
/// Returns after marking the task failed and recording an observation.
///
/// # Examples
///
/// ```rust
/// use codetether_a2a_http::task_state::stream::failure::record;
/// let _operation = record;
/// ```
pub fn record(
    tasks: &DashMap<String, Task>,
    task_id: &str,
    prompt: &str,
    started_at: Instant,
    e: anyhow::Error,
) {
    tracing::error!(target: "codetether_agent::a2a::server", "Stream task {} failed: {}", task_id, e);
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
