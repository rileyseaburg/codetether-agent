//! `message/send`: accept a peer's turn, run it, record the outcome.
//!
//! The turn itself runs through [`super::server_turn::execute_turn`], which
//! prefers the interactive TUI session when one has opted in and falls
//! back to a headless session otherwise. Both `blocking` modes share the
//! same body; only whether the caller awaits it differs.

#[path = "server_message_send_parse.rs"]
mod parse;
#[path = "server_message_send_turn.rs"]
mod turn;

use crate::a2a::types::{JsonRpcError, JsonRpcRequest, MessageSendParams, SendMessageResponse};

use super::{A2AServer, emit_a2a_inbound};

pub(super) async fn handle(
    server: &A2AServer,
    request: JsonRpcRequest,
) -> Result<serde_json::Value, JsonRpcError> {
    let params: MessageSendParams = serde_json::from_value(request.params)
        .map_err(|e| JsonRpcError::invalid_params(format!("Invalid parameters: {e}")))?;
    let (task_id, task) = parse::open_task(&params);
    if crate::a2a::intro::is_intro(&params.message) {
        return super::server_intro::handle_intro(server, &task_id, &params.message, task);
    }
    server.tasks.insert(task_id.clone(), task);
    emit_a2a_inbound(server, &task_id, &params.message);

    let prompt = parse::text_of(&params.message.parts);
    if prompt.is_empty() {
        parse::fail_empty(&server.tasks, &task_id);
        return Err(JsonRpcError::invalid_params("No text content in message"));
    }
    let blocking = parse::blocking(&params);
    let run = turn::run(server.clone(), task_id.clone(), params, prompt, blocking);
    if blocking {
        run.await;
    } else {
        tokio::spawn(run);
    }
    let task = server
        .tasks
        .get(&task_id)
        .ok_or_else(|| JsonRpcError::internal_error(format!("Task disappeared: {task_id}")))?;
    serde_json::to_value(SendMessageResponse::Task(task.value().clone()))
        .map_err(|e| JsonRpcError::internal_error(format!("Serialization error: {e}")))
}
