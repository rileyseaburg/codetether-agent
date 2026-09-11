//! Read structured JSONL progress from a mux-owned agent turn.

use crate::mux::protocol::AgentResponse;

use super::context::ServerContext;

pub(super) async fn apply(
    context: &ServerContext,
    session: &str,
    task_id: String,
    offset: u64,
) -> AgentResponse {
    context
        .tasks
        .read(session, &task_id, offset)
        .await
        .map(
            |(data, next_offset, running, exit_code)| AgentResponse::Output {
                task_id: task_id.clone(),
                data,
                next_offset,
                running,
                exit_code,
            },
        )
        .unwrap_or_else(|error| AgentResponse::Error {
            task_id: Some(task_id),
            message: error.to_string(),
        })
}
