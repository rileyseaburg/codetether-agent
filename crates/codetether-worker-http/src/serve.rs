//! Build the worker routes and serve them on the configured address.
use super::*;

/// Start the worker HTTP server with custom state
///
/// # Arguments
/// `hostname` and `port` select the listener; `state` holds worker channels.
/// # Returns
/// Completes when serving stops.
/// # Errors
/// Returns listener binding or HTTP serving errors.
/// # Examples
/// ```rust,no_run
/// async fn serve() -> anyhow::Result<()> {
/// codetether_worker_http::serve("127.0.0.1", 8080, Default::default()).await
/// }
/// ```
pub async fn serve(hostname: &str, port: u16, state: WorkerServerState) -> Result<()> {
    let addr = format!("{}:{}", hostname, port);

    tracing::info!(target: "codetether_agent::worker_server", "Starting worker HTTP server on http://{}", addr);

    let app = Router::new()
        .route("/health", get(health))
        .route("/ready", get(ready))
        .route("/task", post(receive_task))
        .route("/worker/status", get(worker_status))
        .route("/v1/bus/stream", get(stream_bus_events))
        .route("/v1/bus/publish", post(publish_bus_event))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!(target: "codetether_agent::worker_server", "Worker HTTP server listening on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
