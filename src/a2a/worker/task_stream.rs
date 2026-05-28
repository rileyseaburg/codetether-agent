//! SSE task stream handling for the worker.

use anyhow::Result;
use futures::StreamExt;
use tokio::sync::mpsc;

use super::{
    WorkerTaskRuntime, pending_tasks::poll_pending_tasks, task_dispatch::spawn_task_handler,
};

#[derive(Debug, PartialEq, Eq)]
pub(super) enum StreamDisconnectReason {
    Ended,
    ReadError(String),
}

pub(super) async fn connect_stream(
    runtime: &WorkerTaskRuntime,
    name: &str,
    codebases: &[String],
    task_notify_rx: Option<mpsc::Receiver<String>>,
) -> Result<StreamDisconnectReason> {
    let mut request = runtime
        .client
        .get(format!(
            "{}/v1/worker/tasks/stream?agent_name={}&worker_id={}",
            runtime.server,
            urlencoding::encode(name),
            urlencoding::encode(&runtime.worker_id)
        ))
        .header("Accept", "text/event-stream")
        .header("Accept-Encoding", "identity")
        .header("Cache-Control", "no-cache, no-transform")
        .header("X-Worker-ID", &runtime.worker_id)
        .header("X-Agent-Name", name)
        .header("X-Codebases", codebases.join(","))
        .header("X-Workspaces", codebases.join(","));
    if let Some(token) = &runtime.token {
        request = request.bearer_auth(token);
    }
    let response = request.send().await?;
    if !response.status().is_success() {
        anyhow::bail!("Failed to connect: {}", response.status());
    }
    let mut stream = response.bytes_stream();
    let mut buffer = String::new();
    let mut poll_interval = tokio::time::interval(tokio::time::Duration::from_secs(30));
    let mut task_notify_rx = task_notify_rx;
    poll_interval.tick().await;
    loop {
        tokio::select! {
            task_id = async { if let Some(ref mut rx) = task_notify_rx { rx.recv().await } else { futures::future::pending().await } } => if task_id.is_some() && let Err(error) = poll_pending_tasks(runtime).await { tracing::warn!(error = %error, "Task notification poll failed"); },
            chunk = stream.next() => match chunk { Some(Ok(chunk)) => { buffer.push_str(&String::from_utf8_lossy(&chunk)); while let Some(pos) = buffer.find("\n\n") { let event = buffer[..pos].to_string(); buffer = buffer[pos + 2..].to_string(); if let Some(data) = event.lines().find(|line| line.starts_with("data:")).map(|line| line.trim_start_matches("data:").trim()) && !data.is_empty() && data != "[DONE]" && let Ok(task) = serde_json::from_str::<serde_json::Value>(data) { spawn_task_handler(&task, runtime).await; } } }
                Some(Err(error)) => return Ok(StreamDisconnectReason::ReadError(error.to_string())), None => return Ok(StreamDisconnectReason::Ended), },
            _ = poll_interval.tick() => if let Err(error) = poll_pending_tasks(runtime).await { tracing::warn!(error = %error, "Periodic task poll failed"); },
        }
    }
}
