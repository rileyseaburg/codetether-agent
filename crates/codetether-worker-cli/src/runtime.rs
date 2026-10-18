//! Coordinate native workers with the optional shared-state HTTP task.
use crate::Host;
use codetether_cli_options::A2aArgs;
use codetether_worker_http::WorkerServerState;
/// Execute a worker CLI request with saved credentials and optional HTTP ingress.
/// # Arguments
/// `args` contains the server target, explicit credentials and HTTP settings.
/// # Returns
/// The native worker's completion result; the HTTP task is aborted after return.
/// # Errors
/// Returns the native worker's startup or execution error.
/// # Examples
/// ```no_run
/// async fn worker<H: codetether_worker_cli::Host>(args: codetether_cli_options::A2aArgs) {
///     codetether_worker_cli::execute::<H>(args).await.unwrap();
/// }
/// ```
pub async fn execute<H: Host>(mut args: A2aArgs) -> anyhow::Result<()> {
    crate::credentials::apply(&mut args);
    if args.no_http_server {
        H::run(args).await
    } else {
        let worker_state = WorkerServerState::new();
        let hostname = args.hostname.clone();
        let port = args.port;
        let shared_state = worker_state.clone();
        let http_server_handle = tokio::spawn(async move {
            if let Err(e) = codetether_worker_http::serve(&hostname, port, shared_state).await {
                tracing::error!("Worker HTTP server error: {}", e);
            }
        });
        let result = H::run_with_state(args, worker_state).await;
        http_server_handle.abort();
        result
    }
}
