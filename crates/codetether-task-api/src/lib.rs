//! Task queue HTTP operations and process-local dispatch.
//!
//! Callers supply the in-process queue and bus; route authorization and
//! persistent execution ownership stay outside this crate.
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ListAgentTasksQuery {
    pub status: Option<String>,
    pub agent_type: Option<String>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct CreateAgentTaskRequest {
    pub title: String,
    pub description: String,
    pub agent_type: Option<String>,
    pub model: Option<String>,
    pub priority: Option<i32>,
}
mod list_knative_tasks;
pub use list_knative_tasks::list_knative_tasks;
mod get_knative_task;
pub use get_knative_task::get_knative_task;
mod claim_knative_task;
pub use claim_knative_task::claim_knative_task;
mod complete_knative_task;
pub use complete_knative_task::complete_knative_task;
mod list_agent_tasks;
pub use list_agent_tasks::list_agent_tasks;
mod create_agent_task;
pub use create_agent_task::create_agent_task;
mod get_agent_task;
pub use get_agent_task::get_agent_task;
pub mod dispatch;

/// Worker progress acceptance and publication.
pub mod output_ingest;
/// Accepted completion receipt polling.
pub mod output_poll;
/// Worker terminal transitions and notifications.
pub mod release;

/// Atomic worker task claims.
pub mod worker_claim;
/// Queue-backed worker notification streaming.
pub mod worker_stream;

mod output_stream;
pub use output_stream::stream_task_output;

/// Task route groups over application-owned queue and bus state.
pub mod http;

/// Worker HTTP extraction using the application queue and event bus.
pub mod worker_http;
