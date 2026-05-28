//! Extended task heartbeat support for persistent workers.

use anyhow::{Context, Result};
use reqwest::Client;

pub(super) async fn send_extended_task_heartbeat(
    client: &Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    progress: serde_json::Value,
    lease_seconds: u64,
) -> Result<()> {
    let task_id = progress
        .get("task_id")
        .and_then(|value| value.as_str())
        .filter(|value| !value.is_empty())
        .context("active task progress missing task_id")?
        .to_string();
    let checkpoint_seq = progress
        .get("checkpoints_reached")
        .and_then(|value| value.as_u64())
        .unwrap_or(1)
        .max(1);
    let status_message = progress
        .get("current_checkpoint")
        .and_then(|value| value.as_str())
        .map(ToString::to_string);
    let mut request = client.post(format!("{}/v1/worker/tasks/heartbeat-extended", server));
    if let Some(token) = token {
        request = request.bearer_auth(token);
    }
    let response = request
        .json(&serde_json::json!({ "task_id": &task_id, "worker_id": worker_id, "status_message": status_message, "checkpoint": progress, "checkpoint_seq": checkpoint_seq, "lease_extension_seconds": lease_seconds }))
        .send()
        .await?;
    if !response.status().is_success() {
        anyhow::bail!(
            "extended heartbeat rejected with {}: {}",
            response.status(),
            response.text().await.unwrap_or_default()
        );
    }
    let body = response
        .json::<serde_json::Value>()
        .await
        .unwrap_or_default();
    if body.get("success").and_then(|value| value.as_bool()) == Some(false) {
        tracing::debug!(task_id = %task_id, message = body.get("message").and_then(|value| value.as_str()).unwrap_or("no active run"), "Extended task heartbeat skipped");
    }
    Ok(())
}
