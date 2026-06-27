//! SSE task stream handling for the worker.

use anyhow::Result;
use futures::StreamExt;
use tokio::sync::mpsc;

use crate::a2a::stream::cursor::Cursor;
use crate::a2a::stream::staging::StagingBuffer;
use crate::worker_server::WorkerServerState;

use super::{
    WorkerTaskRuntime,
    pending_tasks::poll_pending_tasks,
    task_stream_buffer::{STAGING_CAP_BYTES, StreamDisconnectReason, cursor_path, ingest_chunk},
    task_stream_error::connect_error,
    task_stream_request::build_stream_request,
};

pub(super) async fn connect_stream(
    runtime: &WorkerTaskRuntime,
    name: &str,
    codebases: &[String],
    task_notify_rx: Option<mpsc::Receiver<String>>,
    server_state: Option<&WorkerServerState>,
) -> Result<StreamDisconnectReason> {
    let mut cursor = Cursor::load(cursor_path(&runtime.worker_id));
    let response = build_stream_request(runtime, name, codebases, cursor.last())
        .send()
        .await?;
    if !response.status().is_success() {
        return Err(connect_error(response).await);
    }
    if let Some(state) = server_state {
        state.set_connected(true).await;
    }
    let mut stream = response.bytes_stream();
    let mut buffer = StagingBuffer::new(STAGING_CAP_BYTES);
    let mut poll_interval = tokio::time::interval(tokio::time::Duration::from_secs(30));
    let mut task_notify_rx = task_notify_rx;
    poll_interval.tick().await;
    loop {
        tokio::select! {
            task_id = async { if let Some(ref mut rx) = task_notify_rx { rx.recv().await } else { futures::future::pending().await } } => if task_id.is_some() && let Err(error) = poll_pending_tasks(runtime).await { tracing::warn!(error = %error, "Task notification poll failed"); },
            chunk = stream.next() => match chunk {
                Some(Ok(chunk)) => if let Err(error) = ingest_chunk(&mut buffer, &chunk, runtime, &mut cursor).await { return Ok(StreamDisconnectReason::ReadError(format!("{error:?}"))); },
                Some(Err(error)) => return Ok(StreamDisconnectReason::ReadError(error.to_string())),
                None => return Ok(StreamDisconnectReason::Ended),
            },
            _ = poll_interval.tick() => if let Err(error) = poll_pending_tasks(runtime).await { tracing::warn!(error = %error, "Periodic task poll failed"); },
        }
    }
}
