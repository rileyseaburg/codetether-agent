//! Execute one accepted `message/send` turn and settle its task.

use std::time::Instant;

use crate::a2a::types::MessageSendParams;

use super::super::{A2AServer, emit_a2a_outbound, server_settle::Settle, server_turn};

pub(super) async fn run(
    server: A2AServer,
    task_id: String,
    params: MessageSendParams,
    prompt: String,
    blocking: bool,
) {
    let started_at = Instant::now();
    let context_id = params.message.context_id.as_deref();
    let from = super::parse::sender_of(&params);
    let outcome =
        server_turn::execute_turn(&server.tasks, &task_id, context_id, &from, &prompt).await;
    let settle = Settle {
        tasks: &server.tasks,
        task_id: &task_id,
        context_id,
        prompt: &prompt,
        blocking,
        elapsed: started_at.elapsed(),
    };
    let message = match outcome {
        Ok(text) => settle.completed(text),
        Err(error) => settle.failed(&error),
    };
    emit_a2a_outbound(&server, &task_id, &message);
}
