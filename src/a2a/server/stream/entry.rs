//! Validate and admit a message/stream request.
use super::*;

pub(in crate::a2a::server) async fn handle_message_stream(
    server: &A2AServer,
    request: JsonRpcRequest,
) -> Result<serde_json::Value, JsonRpcError> {
    // message/stream submits the task for async processing.
    // The client should poll tasks/get for status updates.
    // True SSE streaming requires a dedicated endpoint outside JSON-RPC.

    let params: MessageSendParams = serde_json::from_value(request.params)
        .map_err(|e| JsonRpcError::invalid_params(format!("Invalid parameters: {}", e)))?;

    let (task_id, task) = open::open(&params);

    // Mesh introductions get a canned ack: no session, no LLM call.
    if crate::a2a::intro::is_intro(&params.message) {
        return server_intro::handle_intro(server, &task_id, &params.message, task);
    }

    server.tasks.insert(task_id.clone(), task.clone());
    emit_a2a_inbound(server, &task_id, &params.message);

    // Extract prompt
    let prompt = text::text(&params);

    if prompt.is_empty() {
        if let Some(mut t) = server.tasks.get_mut(&task_id) {
            t.status.state = TaskState::Failed;
            t.status.timestamp = Some(chrono::Utc::now().to_rfc3339());
        }
        return Err(JsonRpcError::invalid_params("No text content in message"));
    }

    spawn::spawn(server, &params, &task_id, prompt);

    // Return task in Working state — client polls tasks/get for completion
    let response = SendMessageResponse::Task(task);
    serde_json::to_value(response)
        .map_err(|e| JsonRpcError::internal_error(format!("Serialization error: {}", e)))
}
