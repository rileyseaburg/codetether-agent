//! Per-attempt connection setup helpers for the worker reconnect loop.

use super::{WorkerContext, start_heartbeat};

/// Start the periodic heartbeat task for the current connection attempt.
pub(super) fn start_connection_heartbeat(context: &WorkerContext) -> tokio::task::JoinHandle<()> {
    start_heartbeat(
        context.task_runtime.client.clone(),
        context.server.clone(),
        context.args.token.clone(),
        context.heartbeat_state.clone(),
        context.processing.clone(),
        context.cognition_heartbeat.clone(),
        context.task_progress.clone(),
    )
}
