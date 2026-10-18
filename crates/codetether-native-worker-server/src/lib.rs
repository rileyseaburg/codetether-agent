//! Application CLI adapter for the shared worker HTTP server.
use anyhow::Result;
use codetether_cli_options::WorkerServerArgs;
pub use codetether_worker_http::WorkerServerState;
/// Start the worker HTTP server with CLI-selected address and shared state.
///
/// # Arguments
/// `args` selects the listening address; `state` holds live worker channels.
/// # Returns
/// Completes when the HTTP server stops.
/// # Errors
/// Returns listener binding or server errors.
/// # Examples
/// ```rust,no_run
/// # async fn example() -> anyhow::Result<()> {
/// let args = codetether_cli_options::WorkerServerArgs { hostname: "127.0.0.1".into(), port: 8080 };
/// codetether_native_worker_server::start_worker_server_with_state(args, Default::default()).await?;
/// # Ok(())
/// # }
/// ```
pub async fn start_worker_server_with_state(
    args: WorkerServerArgs,
    state: WorkerServerState,
) -> Result<()> {
    codetether_worker_http::serve(&args.hostname, args.port, state).await
}
