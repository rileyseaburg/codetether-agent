//! create agent task queue API operation.
use crate::CreateAgentTaskRequest;
use axum::{Json, http::StatusCode};
use codetether_task_queue::{KnativeTask, KnativeTaskQueue};

/// Create a new agent task
pub async fn create_agent_task(
    queue: &KnativeTaskQueue,
    req: CreateAgentTaskRequest,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let task_id = uuid::Uuid::new_v4().to_string();
    let task = KnativeTask {
        task_id: task_id.clone(),
        title: req.title,
        description: req.description,
        agent_type: req.agent_type.unwrap_or_else(|| "build".to_string()),
        model: req.model,
        metadata: None,
        priority: req.priority.unwrap_or(0),
        received_at: chrono::Utc::now(),
        status: "pending".to_string(),
        completion: Default::default(),
    };
    queue.push(task).await;
    Ok(Json(serde_json::json!({
        "task_id": task_id,
        "status": "pending",
    })))
}
