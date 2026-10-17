//! JSON-RPC dispatch and response envelopes.
use super::*;

/// Handle JSON-RPC requests
pub(super) async fn handle_rpc<H: ServerHost>(
    State(server): State<H>,
    Json(request): Json<JsonRpcRequest>,
) -> Result<Json<JsonRpcResponse>, (StatusCode, Json<JsonRpcResponse>)> {
    let request_id = request.id.clone();
    let response = match request.method.as_str() {
        "message/send" => server.send_message(request).await,
        "message/stream" => server.stream_message(request).await,
        "tasks/get" => crate::task_state::handle_tasks_get(server.tasks(), request).await,
        "tasks/cancel" => crate::task_state::handle_tasks_cancel(server.tasks(), request).await,
        _ => Err(JsonRpcError::method_not_found(&request.method)),
    };

    match response {
        Ok(result) => Ok(Json(JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: request_id.clone(),
            result: Some(result),
            error: None,
        })),
        Err(error) => Err((
            StatusCode::OK,
            Json(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request_id,
                result: None,
                error: Some(error),
            }),
        )),
    }
}
