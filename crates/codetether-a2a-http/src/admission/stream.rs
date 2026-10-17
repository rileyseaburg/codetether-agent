//! Validate, dispatch and serialize message/stream requests.
use super::*;
/// Admit a message/stream request through application hooks.
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
/// let _response = codetether_a2a_http::admission::stream(host, request).await;
/// # }
/// ```

pub async fn stream<H: MessageAdmissionHost>(
    server: &H,
    request: JsonRpcRequest,
) -> Result<serde_json::Value, JsonRpcError> {
    // message/stream submits the task for async processing.
    // The client should poll tasks/get for status updates.
    // True SSE streaming requires a dedicated endpoint outside JSON-RPC.

    let params: MessageSendParams = serde_json::from_value(request.params)
        .map_err(|e| JsonRpcError::invalid_params(format!("Invalid parameters: {}", e)))?;

    let (task_id, task) = open::open(&params);

    // Mesh introductions get a canned ack: no session, no LLM call.
    if is_intro(&params.message) {
        return intro::handle_intro(server, &task_id, &params.message, task);
    }

    server.tasks().insert(task_id.clone(), task.clone());
    server.emit_inbound(&task_id, &params.message);

    // Extract prompt
    let prompt = text::text(&params);

    if prompt.is_empty() {
        if let Some(mut t) = server.tasks().get_mut(&task_id) {
            t.status.state = TaskState::Failed;
            t.status.timestamp = Some(chrono::Utc::now().to_rfc3339());
        }
        return Err(JsonRpcError::invalid_params("No text content in message"));
    }

    server.spawn_stream(&params, &task_id, prompt);

    // Return task in Working state — client polls tasks/get for completion
    let response = SendMessageResponse::Task(task);
    serde_json::to_value(response)
        .map_err(|e| JsonRpcError::internal_error(format!("Serialization error: {}", e)))
}
