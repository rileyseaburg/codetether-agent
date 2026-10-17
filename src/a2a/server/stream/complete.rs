//! Store stream success and record completion telemetry.
use super::*;
pub(super) fn complete(
    tasks: &DashMap<String, Task>,
    task_id: &String,
    context_id: Option<String>,
    prompt: &str,
    started_at: Instant,
    result_text: String,
) {
    let response_message = reply::message(task_id, context_id, &result_text);

    let artifact = artifact::artifact(&result_text);

    if let Some(mut t) = tasks.get_mut(task_id) {
        t.status.state = TaskState::Completed;
        t.status.message = Some(response_message.clone());
        t.status.timestamp = Some(chrono::Utc::now().to_rfc3339());
        t.artifacts.push(artifact.clone());
        t.history.push(response_message);

        completion_events::record(&t, task_id, artifact);
    }

    record_a2a_message_telemetry(
        "a2a_message_stream",
        task_id,
        false,
        &prompt,
        started_at.elapsed(),
        true,
        Some(result_text),
        None,
    );
}
