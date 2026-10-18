//! agent request adapters over shared task operations.
use super::HttpState;
use crate::{CreateAgentTaskRequest, ListAgentTasksQuery};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use codetether_task_queue::KnativeTask;

// ── Agent task compatibility surface ─────────────────────────────────────
// These handlers provide the /v1/agent/tasks/* surface the marketing-site
// dashboard expects, backed by the existing KnativeTaskQueue.

/// List all agent tasks (wraps Knative task queue)
pub(super) async fn list_agent_tasks<S: HttpState>(
    State(state): State<S>,
    Query(params): Query<ListAgentTasksQuery>,
) -> Json<serde_json::Value> {
    crate::list_agent_tasks(state.queue(), params).await
}

/// Create a new agent task
pub(super) async fn create_agent_task<S: HttpState>(
    State(state): State<S>,
    Json(req): Json<CreateAgentTaskRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    crate::create_agent_task(state.queue(), req).await
}

/// Get a single agent task by ID
pub(super) async fn get_agent_task<S: HttpState>(
    State(state): State<S>,
    Path(task_id): Path<String>,
) -> Result<Json<KnativeTask>, (StatusCode, String)> {
    crate::get_agent_task(state.queue(), task_id).await
}
