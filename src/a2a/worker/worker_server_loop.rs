//! Reconnection loop for HTTP-server-integrated workers.

use anyhow::Result;
use tokio::sync::mpsc;

use crate::worker_server::WorkerServerState;

use super::{
    StreamDisconnectReason, WorkerContext, connect_stream, fetch_pending_tasks,
    register_current_worker, start_heartbeat,
};

pub(super) async fn run_worker_server_loop(
    context: WorkerContext,
    server_state: WorkerServerState,
) -> Result<()> {
    loop {
        let codebases = context.shared_codebases.lock().await.clone();
        let (notify_tx, notify_rx) = mpsc::channel::<String>(32);
        server_state.set_task_notification_channel(notify_tx).await;
        server_state.set_connected(false).await;
        if let Err(error) = register_current_worker(&context).await {
            tracing::warn!(error = %error, "Failed to re-register worker on reconnection");
        }
        if let Err(error) = fetch_pending_tasks(&context.task_runtime).await {
            tracing::warn!(error = %error, "Reconnect task fetch failed");
        }
        let heartbeat = start_heartbeat(
            context.task_runtime.client.clone(),
            context.server.clone(),
            context.args.token.clone(),
            context.heartbeat_state.clone(),
            context.processing.clone(),
            context.cognition_heartbeat.clone(),
            context.task_progress.clone(),
        );
        match connect_stream(
            &context.task_runtime,
            &context.name,
            &codebases,
            Some(notify_rx),
            Some(&server_state),
        )
        .await
        {
            Ok(StreamDisconnectReason::Ended) => tracing::warn!("Stream ended, reconnecting..."),
            Ok(StreamDisconnectReason::ReadError(error)) => {
                tracing::warn!(error = %error, "Stream read failed, reconnecting...")
            }
            Err(error) => tracing::error!("Stream error: {}, reconnecting...", error),
        }
        server_state.set_connected(false).await;
        heartbeat.abort();
        tracing::debug!("Heartbeat cancelled for reconnection");
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}
