//! SSE task stream handling for the worker.

use anyhow::Result;
use futures::StreamExt;
use tokio::sync::mpsc;

use crate::worker_server::WorkerServerState;

use super::{
    WorkerTaskRuntime, pending_tasks::poll_pending_tasks, task_dispatch::spawn_task_handler,
    task_stream_request::build_stream_request,
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
    server_state: Option<&WorkerServerState>,
) -> Result<StreamDisconnectReason> {
    let response = build_stream_request(runtime, name, codebases)
        .send()
        .await?;
    if !response.status().is_success() {
        let status = response.status();
        let body = response
            .text()
            .await
            .unwrap_or_else(|error| format!("<failed to read response body: {error}>"));
        anyhow::bail!(
            "Failed to connect task stream: status={} body={}",
            status,
            summarize_response_body(&body)
        );
    }
    if let Some(state) = server_state {
        state.set_connected(true).await;
    }
    let mut stream = response.bytes_stream();
    let mut buffer = Vec::<u8>::new();
    let mut poll_interval = tokio::time::interval(tokio::time::Duration::from_secs(30));
    let mut task_notify_rx = task_notify_rx;
    poll_interval.tick().await;
    loop {
        tokio::select! {
            task_id = async { if let Some(ref mut rx) = task_notify_rx { rx.recv().await } else { futures::future::pending().await } } => if task_id.is_some() && let Err(error) = poll_pending_tasks(runtime).await { tracing::warn!(error = %error, "Task notification poll failed"); },
            chunk = stream.next() => match chunk {
                Some(Ok(chunk)) => { buffer.extend_from_slice(&chunk); process_buffer(&mut buffer, runtime).await; }
                Some(Err(error)) => return Ok(StreamDisconnectReason::ReadError(error.to_string())),
                None => return Ok(StreamDisconnectReason::Ended),
            },
            _ = poll_interval.tick() => if let Err(error) = poll_pending_tasks(runtime).await { tracing::warn!(error = %error, "Periodic task poll failed"); },
        }
    }
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

async fn process_buffer(buffer: &mut Vec<u8>, runtime: &WorkerTaskRuntime) {
    while let Some(pos) = buffer.windows(2).position(|w| w == b"\n\n") {
        let event_bytes = buffer[..pos].to_vec();
        buffer.drain(..pos + 2);
        let Ok(event_str) = std::str::from_utf8(&event_bytes) else {
            continue;
        };
        if let Some(data) = event_str
            .lines()
            .find(|l| l.starts_with("data:"))
            .map(|l| l.trim_start_matches("data:").trim())
            && !data.is_empty()
            && data != "[DONE]"
            && let Ok(task) = serde_json::from_str::<serde_json::Value>(data)
        {
            spawn_task_handler(&task, runtime).await;
        }
    }
}
