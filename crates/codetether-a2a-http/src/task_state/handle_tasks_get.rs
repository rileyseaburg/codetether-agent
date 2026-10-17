//! handle tasks get JSON-RPC handler.
use super::*;

pub async fn handle_tasks_get(
    tasks: &DashMap<String, Task>,
    request: JsonRpcRequest,
) -> Result<serde_json::Value, JsonRpcError> {
    let params: TaskQueryParams = serde_json::from_value(request.params)
        .map_err(|e| JsonRpcError::invalid_params(format!("Invalid parameters: {}", e)))?;

    let task = tasks.get(&params.id).ok_or_else(|| JsonRpcError {
        code: TASK_NOT_FOUND,
        message: format!("Task not found: {}", params.id),
        data: None,
    })?;

    serde_json::to_value(task.value().clone())
        .map_err(|e| JsonRpcError::internal_error(format!("Serialization error: {}", e)))
}
