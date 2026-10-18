//! A2A startup, native state assembly and HTTP listener lifecycle.
use crate::{Host, prepare, transport};
use std::time::Instant;
/// Initialize application services and serve HTTP until the listener exits.
/// # Arguments
/// * `hostname` - Configured HTTP and gRPC interface hostname.
/// * `port` - Configured HTTP listener port.
/// * `version` - Executable package version for the server fingerprint.
/// # Returns
/// Unit when the HTTP server exits normally.
/// # Errors
/// Propagates configuration, gRPC address, HTTP bind and serving failures.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_server_runtime::Host>() -> anyhow::Result<()> {
/// codetether_server_runtime::serve::<H>("127.0.0.1", 8080, "1.0.0").await
/// # }
/// ```
pub async fn serve<H: Host>(hostname: &str, port: u16, version: &str) -> anyhow::Result<()> {
    let t0 = Instant::now();
    tracing::info!("[startup] begin");
    let services = prepare::prepare::<H>(t0).await?;
    tracing::info!(
        elapsed_ms = t0.elapsed().as_millis(),
        "[startup] building routes"
    );
    let addr = format!("{hostname}:{port}");
    let (agent_card, a2a_router, a2a_nested_router) = H::a2a(&addr, services.bus.clone());
    codetether_a2a_grpc::listener::spawn(hostname, agent_card, services.bus.clone())?;
    let parts = services.into_parts(version);
    let tool_registry = parts.tool_registry.clone();
    let state = H::state(parts);
    codetether_tool_discovery::spawn_reaper(tool_registry);
    let app = H::router(&state, a2a_router, a2a_nested_router);
    transport::serve(&addr, app, t0).await?;
    Ok(())
}
