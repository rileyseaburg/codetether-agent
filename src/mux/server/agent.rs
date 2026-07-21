//! Authenticated structured agent-turn request handling.

use crate::mux::protocol::{AgentRequest, AgentResponse, ServerResponse};

use super::context::ServerContext;

pub(super) async fn apply(context: &ServerContext, request: AgentRequest) -> ServerResponse {
    let response = match request {
        AgentRequest::Start {
            task_id,
            prompt,
            session_id,
            max_steps,
            tool_profile,
        } => {
            super::agent_start::apply(
                context,
                task_id,
                prompt,
                session_id,
                max_steps,
                tool_profile,
            )
            .await
        }
        AgentRequest::Read { task_id, offset } => {
            super::agent_read::apply(context, task_id, offset).await
        }
        AgentRequest::Cancel { task_id } => context
            .tasks
            .cancel(&task_id)
            .map(|()| AgentResponse::Cancelled {
                task_id: task_id.clone(),
            })
            .unwrap_or_else(|error| AgentResponse::Error {
                task_id: Some(task_id),
                message: error.to_string(),
            }),
    };
    ServerResponse::Agent { response }
}
