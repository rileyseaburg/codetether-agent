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
    codetether_a2a_http::worker::pending_tasks(
        &runtime.client,
        &runtime.server,
        &runtime.token,
        &runtime.agent_name,
    )
    .await
}
