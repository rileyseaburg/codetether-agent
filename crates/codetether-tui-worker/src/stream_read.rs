//! Decode and deliver task frames, stopping when the receiver closes.
use codetether_worker_events::{IncomingTask, SseBuffer};
use futures::StreamExt;
use reqwest::Response;
use tokio::sync::mpsc;
pub(crate) async fn read(res: Response, task_tx: &mpsc::Sender<IncomingTask>) -> bool {
    tracing::info!("Connected to A2A task stream");
    let mut stream = res.bytes_stream();
    let mut buffer = SseBuffer::new();

    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(bytes) => {
                let tasks = match buffer.push(&bytes) {
                    Ok(tasks) => tasks,
                    Err(error) => {
                        tracing::warn!(%error, "Rejected A2A SSE frame");
                        break;
                    }
                };
                for incoming in tasks {
                    if task_tx.send(incoming).await.is_err() {
                        tracing::warn!("Task receiver dropped, stopping SSE stream");
                        return true;
                    }
                }
            }
            Err(e) => {
                tracing::warn!("SSE stream error: {}", e);
                break;
            }
        }
    }
    false
}
