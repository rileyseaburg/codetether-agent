//! Backward-compatible output polling response with accepted receipt fields.

use crate::server::KnativeTask;

pub(super) fn from_task(task: &KnativeTask) -> serde_json::Value {
    let mut response = serde_json::json!({
        "task_id": task.task_id,
        "status": task.status,
        "title": task.title,
        "output": task.completion.result,
    });
    for (field, value) in [
        (
            "result",
            task.completion
                .result
                .as_ref()
                .map(|value| serde_json::json!(value)),
        ),
        (
            "error",
            task.completion
                .error
                .as_ref()
                .map(|value| serde_json::json!(value)),
        ),
        (
            "session_id",
            task.completion
                .session_id
                .as_ref()
                .map(|value| serde_json::json!(value)),
        ),
        ("diagnostics", task.completion.diagnostics.clone()),
    ] {
        if let Some(value) = value {
            response[field] = value;
        }
    }
    response
}
