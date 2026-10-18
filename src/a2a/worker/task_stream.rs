//! Native binding for shared worker stream consumption.
mod host;
use super::WorkerTaskRuntime;
use crate::worker_server::WorkerServerState;
use codetether_a2a_stream::ingest::StreamDisconnectReason;
use tokio::sync::mpsc;
pub(super) async fn connect_stream(
    runtime: &WorkerTaskRuntime,
    name: &str,
    codebases: &[String],
    task_notify_rx: Option<mpsc::Receiver<String>>,
    server_state: Option<&WorkerServerState>,
) -> anyhow::Result<StreamDisconnectReason> {
    let host = host::Native {
        runtime,
        name,
        codebases,
        server_state,
    };
    codetether_a2a_stream::worker_connection::connect_stream(
        &host,
        &runtime.worker_id,
        task_notify_rx,
    )
    .await
}
