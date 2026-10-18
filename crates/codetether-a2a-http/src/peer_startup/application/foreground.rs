//! Foreground peer startup with optional archival and Ctrl-C shutdown.
use crate::peer_startup::{SpawnOptions, StartupHooks, prepare_a2a, serve_foreground};
use codetether_bus_core::AgentBus;
/// Run a peer process using application card and router construction.
/// # Arguments
/// `options` converts to peer options after archival setup; `hooks` supplies application bindings.
/// # Returns
/// Completes after the foreground server shuts down.
/// # Errors
/// Propagates preparation and serving errors.
/// # Examples
/// ```no_run
/// # async fn example(hooks: codetether_a2a_http::peer_startup::StartupHooks) -> anyhow::Result<()> {
/// codetether_a2a_http::peer_startup::application::run(
///     codetether_a2a_http::peer_startup::SpawnOptions::auto(), hooks,
/// ).await
/// # }
/// ```
pub async fn run(options: impl Into<SpawnOptions>, hooks: StartupHooks) -> anyhow::Result<()> {
    let bus = AgentBus::new().into_arc();
    codetether_bus_s3::spawn_bus_s3_sink(bus.clone());
    let prep = prepare_a2a(options.into(), bus, hooks).await?;
    serve_foreground(prep, shutdown_signal()).await
}
async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
    tracing::info!(target: "codetether_agent::a2a::spawn", "Shutdown signal received");
}
