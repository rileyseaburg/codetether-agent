//! Enqueue a post-clone task via the server API.

use anyhow::{Context, Result};
use reqwest::Client;

pub(super) async fn enqueue_post_clone_task(
    client: &Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    workspace_id: &str,
    metadata: &serde_json::Map<String, serde_json::Value>,
) -> Result<()> {
    let Some(task) = metadata
        .get("post_clone_task")
        .and_then(|value| value.as_object())
    else {
        return Ok(());
    };
    let title = task.get("title").and_then(|v| v.as_str()).filter(|v| !v.trim().is_empty()).ok_or_else(|| anyhow::anyhow!("post_clone_task is missing title"))?;
    let prompt = task.get("prompt").and_then(|v| v.as_str()).filter(|v| !v.trim().is_empty()).ok_or_else(|| anyhow::anyhow!("post_clone_task is missing prompt"))?;
    let mut task_metadata = task
        .get("metadata")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}));
    if let Some(obj) = task_metadata.as_object_mut() {
        obj.entry("target_worker_id".to_string())
            .or_insert_with(|| serde_json::Value::String(worker_id.to_string()));
    }
    let mut request = client.post(format!(
        "{}/v1/agent/workspaces/{workspace_id}/tasks",
        server.trim_end_matches('/')
    ));
    if let Some(token) = token {
        request = request.bearer_auth(token);
    }
    let response = request
        .json(&serde_json::json!({ "title": title, "prompt": prompt, "agent_type": task.get("agent_type").and_then(|value| value.as_str()).unwrap_or("build"), "metadata": task_metadata }))
        .send()
        .await
        .context("Failed to enqueue post-clone task")?;
    if response.status().is_success() {
        return Ok(());
    }
    anyhow::bail!(
        "Failed to enqueue post-clone task ({}): {}",
        response.status(),
        response.text().await.unwrap_or_default()
    )
}
