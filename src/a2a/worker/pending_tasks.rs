//! Pending-task polling helpers for the worker.

use anyhow::Result;

use super::{WorkerTaskRuntime, task_dispatch::spawn_task_handler};

pub(super) async fn fetch_pending_tasks(runtime: &WorkerTaskRuntime) -> Result<()> {
    let tasks = pending_tasks(runtime).await?;
    tracing::info!(count = tasks.len(), "Found pending tasks");
    for task in tasks {
        spawn_task_handler(&task, runtime).await;
    }
    Ok(())
}

pub(super) async fn poll_pending_tasks(runtime: &WorkerTaskRuntime) -> Result<()> {
    let tasks = pending_tasks(runtime).await?;
    if !tasks.is_empty() {
        tracing::debug!(count = tasks.len(), "Poll found pending tasks");
    }
    for task in &tasks {
        spawn_task_handler(task, runtime).await;
    }
    Ok(())
}

async fn pending_tasks(runtime: &WorkerTaskRuntime) -> Result<Vec<serde_json::Value>> {
    let mut url = format!("{}/v1/agent/tasks?status=pending", runtime.server);
    if !runtime.agent_name.is_empty() {
        url = format!(
            "{}&agent_name={}",
            url,
            urlencoding::encode(&runtime.agent_name)
        );
    }
    let mut request = runtime.client.get(&url);
    if let Some(token) = &runtime.token {
        request = request.bearer_auth(token);
    }
    let response = request.send().await?;
    if !response.status().is_success() {
        let status = response.status();
        let body = response
            .text()
            .await
            .unwrap_or_else(|error| format!("<failed to read response body: {error}>"));
        tracing::warn!(
            %status,
            body = %summarize_response_body(&body),
            "Failed to fetch pending tasks"
        );
        return Ok(Vec::new());
    }
    let data: serde_json::Value = response.json().await?;
    Ok(data
        .as_array()
        .cloned()
        .or_else(|| data["tasks"].as_array().cloned())
        .unwrap_or_default())
}

fn summarize_response_body(body: &str) -> String {
    const MAX_BODY_CHARS: usize = 512;
    let mut summary = body.split_whitespace().collect::<Vec<_>>().join(" ");
    if summary.chars().count() > MAX_BODY_CHARS {
        summary = summary.chars().take(MAX_BODY_CHARS).collect::<String>();
        summary.push_str("...");
    }
    summary
}
