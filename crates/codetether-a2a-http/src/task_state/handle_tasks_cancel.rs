//! handle tasks cancel JSON-RPC handler.
use super::*;

pub async fn handle_tasks_cancel(
    tasks: &DashMap<String, Task>,
    request: JsonRpcRequest,
) -> Result<serde_json::Value, JsonRpcError> {
    let params: TaskQueryParams = serde_json::from_value(request.params)
        .map_err(|e| JsonRpcError::invalid_params(format!("Invalid parameters: {}", e)))?;

    let mut task = tasks.get_mut(&params.id).ok_or_else(|| JsonRpcError {
        code: TASK_NOT_FOUND,
        message: format!("Task not found: {}", params.id),
        data: None,
    })?;

    if !task.status.state.is_active() {
        return Err(JsonRpcError {
            code: TASK_NOT_CANCELABLE,
            message: "Task is already in a terminal state".to_string(),
            data: None,
        });
    }

    task.status.state = TaskState::Cancelled;
    task.status.timestamp = Some(chrono::Utc::now().to_rfc3339());

    serde_json::to_value(task.value().clone())
        .map_err(|e| JsonRpcError::internal_error(format!("Serialization error: {}", e)))
}
