//! Reconnection loop for HTTP-server-integrated workers.

use anyhow::Result;
use tokio::sync::mpsc;

use crate::a2a::stream::breaker::CircuitBreaker;
use crate::worker_server::WorkerServerState;

use super::{
    WorkerContext, connect_stream,
    connection_setup::start_connection_heartbeat,
    fetch_pending_tasks,
    reconnect_lifecycle::{apply_lifecycle, log_outcome, make_backoff},
    register_current_worker,
    transport_probe::spawn_transport_probe,
};

pub(super) async fn run_worker_server_loop(
    context: WorkerContext,
    server_state: WorkerServerState,
) -> Result<()> {
    let mut backoff = make_backoff();
    let mut breaker = CircuitBreaker::new(5);
    let _probe = spawn_transport_probe(&context.server);
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
        let heartbeat = start_connection_heartbeat(&context);
        let outcome = connect_stream(
            &context.task_runtime,
            &context.name,
            &codebases,
            Some(notify_rx),
            Some(&server_state),
        )
        .await;
        let connected = log_outcome(&outcome);
        server_state.set_connected(false).await;
        heartbeat.abort();
        apply_lifecycle(&mut backoff, &mut breaker, connected).await;
    }
}
