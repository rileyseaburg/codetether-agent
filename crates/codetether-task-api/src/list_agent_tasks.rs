//! list agent tasks queue API operation.
use crate::ListAgentTasksQuery;
use axum::Json;
use codetether_task_queue::KnativeTaskQueue;

// ── Agent task compatibility surface ─────────────────────────────────────
// These handlers provide the /v1/agent/tasks/* surface the marketing-site
// dashboard expects, backed by the existing KnativeTaskQueue.

/// List all agent tasks (wraps Knative task queue)
pub async fn list_agent_tasks(
    queue: &KnativeTaskQueue,
    params: ListAgentTasksQuery,
) -> Json<serde_json::Value> {
    let tasks = queue.list().await;
    let filtered: Vec<_> = tasks
        .into_iter()
        .filter(|t| params.status.as_ref().is_none_or(|s| t.status == *s))
        .filter(|t| {
            params
                .agent_type
                .as_ref()
                .is_none_or(|a| t.agent_type == *a)
        })
        .collect();
    Json(serde_json::json!({ "tasks": filtered, "total": filtered.len() }))
}
