//! Validate, dispatch and serialize message/send requests.
use super::*;
/// Admit a message/send request through application hooks.
///
/// # Arguments
///
/// * `server` - Application task table, bus and execution hooks.
/// * `request` - JSON-RPC request containing message parameters.
///
/// # Returns
///
/// The serialized admitted task, or an introduction acknowledgement.
///
/// # Errors
///
/// Returns parameter, prompt validation, missing-task or serialization errors.
///
/// # Examples
///
/// ```rust,no_run
/// # async fn demo<H: codetether_a2a_http::admission::MessageAdmissionHost>(
/// # host: &H, request: codetether_a2a_core::JsonRpcRequest) {
/// let _response = codetether_a2a_http::admission::send(host, request).await;
/// # }
/// ```

pub async fn send<H: MessageAdmissionHost>(
    server: &H,
    request: JsonRpcRequest,
) -> Result<serde_json::Value, JsonRpcError> {
    let params: MessageSendParams = serde_json::from_value(request.params)
        .map_err(|e| JsonRpcError::invalid_params(format!("Invalid parameters: {e}")))?;
    let (task_id, task) = parse::open_task(&params);
    if is_intro(&params.message) {
        return intro::handle_intro(server, &task_id, &params.message, task);
    }
    server.tasks().insert(task_id.clone(), task);
    server.emit_inbound(&task_id, &params.message);

    let prompt = parse::text_of(&params.message.parts);
    if prompt.is_empty() {
        parse::fail_empty(&server.tasks(), &task_id);
        return Err(JsonRpcError::invalid_params("No text content in message"));
    }
    let blocking = parse::blocking(&params);
    let run = H::clone(server).execute_send(task_id.clone(), params, prompt, blocking);
    if blocking {
        run.await;
    } else {
        tokio::spawn(run);
    }
    let task = server
        .tasks()
        .get(&task_id)
        .ok_or_else(|| JsonRpcError::internal_error(format!("Task disappeared: {task_id}")))?;
    serde_json::to_value(SendMessageResponse::Task(task.value().clone()))
        .map_err(|e| JsonRpcError::internal_error(format!("Serialization error: {e}")))
}
