//! Native binding for shared worker stream consumption.
mod host;
use crate::{WorkerTaskRuntime, pending::Dispatcher};
use std::marker::PhantomData;
mod request;
use codetether_a2a_stream::ingest::StreamDisconnectReason;
use codetether_worker_http::WorkerServerState;
use tokio::sync::mpsc;
pub async fn connect_stream<D: Dispatcher>(
    runtime: &WorkerTaskRuntime,
    name: &str,
    codebases: &[String],
    task_notify_rx: Option<mpsc::Receiver<String>>,
    server_state: Option<&WorkerServerState>,
) -> anyhow::Result<StreamDisconnectReason> {
    let host = host::Native::<D> {
        runtime,
        name,
        codebases,
        server_state,
        dispatcher: PhantomData,
    };
    codetether_a2a_stream::worker_connection::connect_stream(
        &host,
        &runtime.worker_id,
        task_notify_rx,
    )
    .await
}
