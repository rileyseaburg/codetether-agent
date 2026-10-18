//! Native worker lifecycle entrypoints.
//!
//! [`run`] initializes a standalone worker, bootstraps registration and pending
//! tasks, then runs its reconnect loop. [`run_with_state`] also attaches the HTTP
//! server state and initializes its connection status. Applications supply
//! catalog and peer [`Hooks`] and a task [`Dispatcher`].
use crate::{
    bootstrap::bootstrap_worker,
    initialization::{Hooks, initialize},
    pending::Dispatcher,
    reconnect::{run_worker_loop, run_worker_server_loop},
};
use anyhow::Result;
use codetether_cli_options::A2aArgs;
use codetether_worker_http::WorkerServerState;

/// Initialize, bootstrap and run a standalone worker.
///
/// # Arguments
/// * `args` - Worker options passed unchanged to initialization.
///
/// # Returns
/// Completion of the standalone worker reconnect loop.
///
/// # Errors
/// Propagates errors from initialization, bootstrap and the reconnect loop.
///
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_native_worker::initialization::Hooks, D: codetether_native_worker::pending::Dispatcher>(args: codetether_cli_options::A2aArgs) -> anyhow::Result<()> {
/// codetether_native_worker::entrypoints::run::<H, D>(args).await
/// # }
/// ```
pub async fn run<H: Hooks, D: Dispatcher>(args: A2aArgs) -> Result<()> {
    let context = initialize::<H>(args).await?;
    bootstrap_worker::<D>(&context).await?;
    run_worker_loop::<D>(context).await
}

/// Initialize and run a worker attached to HTTP server state.
///
/// # Arguments
/// * `args` - Worker options passed unchanged to initialization.
/// * `server_state` - State attached before bootstrap and updated by reconnection.
///
/// # Returns
/// Completion of the HTTP-integrated worker reconnect loop.
///
/// # Errors
/// Propagates errors from initialization, bootstrap and the reconnect loop.
///
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_native_worker::initialization::Hooks, D: codetether_native_worker::pending::Dispatcher>(args: codetether_cli_options::A2aArgs, state: codetether_worker_http::WorkerServerState) -> anyhow::Result<()> {
/// codetether_native_worker::entrypoints::run_with_state::<H, D>(args, state).await
/// # }
/// ```
pub async fn run_with_state<H: Hooks, D: Dispatcher>(
    args: A2aArgs,
    server_state: WorkerServerState,
) -> Result<()> {
    let context = initialize::<H>(args).await?;
    server_state.attach_context(&context).await;
    bootstrap_worker::<D>(&context).await?;
    server_state.set_connected(false).await;
    run_worker_server_loop::<D>(context, server_state).await
}
