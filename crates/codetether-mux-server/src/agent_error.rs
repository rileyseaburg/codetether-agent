//! Error response mapping for agent startup.
use codetether_mux_protocol::AgentResponse;

pub(super) fn error(task_id: String, error: anyhow::Error) -> AgentResponse {
    AgentResponse::Error {
        task_id: Some(task_id),
        message: error.to_string(),
    }
}
