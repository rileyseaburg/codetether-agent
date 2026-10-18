//! Pending-task retrieval bound to the native worker transport.
//!
//! [fetch_pending_tasks] and [poll_pending_tasks] retain shared ordered dispatch.
//! [Dispatcher] connects each returned payload to the application claim handler.
use crate::WorkerTaskRuntime;
use serde_json::Value;
use std::{future::Future, marker::PhantomData};
mod host;
/// Application task dispatch after the worker has retrieved pending payloads.
/// # Implementors
/// Native applications forward to their existing claim-gated task handler.
pub trait Dispatcher {
    /// Dispatch one task against the shared worker runtime.
    fn dispatch(task: &Value, runtime: &WorkerTaskRuntime) -> impl Future<Output = ()> + Send;
}
/// Retrieve startup/reconnect tasks and dispatch them in their returned order.
/// # Arguments
/// * `runtime` - Existing HTTP connection, identity and task ownership state.
/// # Returns
/// Unit after every pending payload is dispatched.
/// # Errors
/// Returns HTTP or response-decoding failures from pending-task retrieval.
pub async fn fetch_pending_tasks<D: Dispatcher>(runtime: &WorkerTaskRuntime) -> anyhow::Result<()> {
    codetether_a2a_worker_core::dispatch::pending::fetch_pending_tasks(&host::Native::<D> {
        runtime,
        dispatcher: PhantomData,
    })
    .await
}
/// Poll pending tasks, retaining the shared nonempty-result logging policy.
/// # Arguments
/// * `runtime` - Existing worker state.
/// # Returns
/// Unit after dispatching the returned payloads.
/// # Errors
/// Returns pending-task transport or decoding failures.
pub async fn poll_pending_tasks<D: Dispatcher>(runtime: &WorkerTaskRuntime) -> anyhow::Result<()> {
    codetether_a2a_worker_core::dispatch::pending::poll_pending_tasks(&host::Native::<D> {
        runtime,
        dispatcher: PhantomData,
    })
    .await
}
