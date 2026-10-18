use super::Host;
use crate::{
    cursor::Cursor,
    ingest::{STAGING_CAP_BYTES, StreamDisconnectReason},
    staging::StagingBuffer,
};
use anyhow::Result;
use futures::StreamExt;
use tokio::sync::mpsc;
/// Consume a resumable worker SSE connection and reconcile pending tasks.
///
/// # Arguments
/// * `host` — Request, task-dispatch and connection-status services.
/// * `worker_id` — Stable identity used to load the durable cursor.
/// * `task_notify_rx` — Optional HTTP-server task notifications.
/// # Returns
/// The reason the byte stream ended or could no longer be consumed.
/// # Errors
/// Propagates request/HTTP failures; ingestion failures become read-error outcomes.
/// # Examples
/// ```ignore
/// let outcome = connect_stream(&worker, "worker-id", notifications).await?;
/// ```
pub async fn connect_stream<H: Host>(
    host: &H,
    worker_id: &str,
    task_notify_rx: Option<mpsc::Receiver<String>>,
) -> Result<StreamDisconnectReason> {
    let mut cursor = Cursor::load(super::cursor_path(worker_id));
    let response = host.request(cursor.last()).send().await?;
    if !response.status().is_success() {
        return Err(host.connect_error(response).await);
    }
    host.connected().await;
    let mut stream = response.bytes_stream();
    let mut buffer = StagingBuffer::new(STAGING_CAP_BYTES);
    let mut poll_interval = tokio::time::interval(tokio::time::Duration::from_secs(30));
    let mut task_notify_rx = task_notify_rx;
    poll_interval.tick().await;
    loop {
        tokio::select! {
            task_id = async { if let Some(ref mut rx) = task_notify_rx { rx.recv().await } else { futures::future::pending().await } } => if task_id.is_some() && let Err(error) = host.reconcile().await { tracing::warn!(error = %error, "Task notification poll failed"); },
            chunk = stream.next() => match chunk {
                Some(Ok(chunk)) => if let Err(error) = super::ingest::ingest_chunk(&mut buffer, &chunk, host, &mut cursor).await { return Ok(StreamDisconnectReason::ReadError(format!("{error:?}"))); },
                Some(Err(error)) => return Ok(StreamDisconnectReason::ReadError(error.to_string())),
                None => return Ok(StreamDisconnectReason::Ended),
            },
            _ = poll_interval.tick() => if let Err(error) = host.reconcile().await { tracing::warn!(error = %error, "Periodic task poll failed"); },
        }
    }
}
