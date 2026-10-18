//! HTTP socket binding and serving lifecycle.
use axum::Router;
use std::time::Instant;
pub(crate) async fn serve(addr: &str, app: Router, t0: Instant) -> anyhow::Result<()> {
    tracing::info!(
        elapsed_ms = t0.elapsed().as_millis(),
        "[startup] router built, binding"
    );
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!(
        elapsed_ms = t0.elapsed().as_millis(),
        "[startup] listening on http://{}",
        addr
    );
    axum::serve(listener, app).await?;
    Ok(())
}
