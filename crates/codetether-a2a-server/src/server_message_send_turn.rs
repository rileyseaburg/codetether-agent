//! Execute one accepted `message/send` turn and settle its task.

use std::time::Instant;

use codetether_a2a_core::MessageSendParams;

use super::{A2AServer, Application, emit_a2a_outbound, server_turn};
use codetether_a2a_http::task_state::settlement::Settle;

pub(super) async fn run<H: Application>(
    server: A2AServer<H>,
    task_id: String,
    params: MessageSendParams,
    prompt: String,
    blocking: bool,
) {
    let started_at = Instant::now();
    let context_id = params.message.context_id.as_deref();
    let from = codetether_a2a_http::task_state::sender_of(&params);
    let outcome =
        server_turn::execute_turn::<H>(&server.tasks, &task_id, context_id, &from, &prompt).await;
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
