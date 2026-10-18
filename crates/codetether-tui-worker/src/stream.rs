//! Reconnect task-stream attempts at the existing fixed interval.
use crate::identity::Identity;
use codetether_worker_events::IncomingTask;
use tokio::sync::mpsc;
pub(crate) async fn run(id: Identity, task_tx: mpsc::Sender<IncomingTask>) {
    loop {
        match crate::stream_request::build(&id).send().await {
            Ok(res) if res.status().is_success() => {
                if crate::stream_read::read(res, &task_tx).await {
                    return;
                }
            }
            Ok(res) => {
                tracing::warn!("Failed to connect to task stream: {}", res.status());
            }
            Err(e) => {
                tracing::warn!("Failed to connect to task stream: {}", e);
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}
