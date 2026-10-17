//! Peer startup command.
use super::*;

/// Run the `codetether spawn` command. Owns its own bus and S3 sink, blocks
/// on ctrl_c with graceful shutdown.
pub async fn run(args: SpawnArgs) -> Result<()> {
    let bus = AgentBus::new().into_arc();

    // Auto-start S3 sink if MinIO is configured
    crate::bus::s3_sink::spawn_bus_s3_sink(bus.clone());

    let prep = prepare_a2a(SpawnOptions::from_spawn_args(&args), bus).await?;

    codetether_a2a_http::peer_startup::serve_foreground(prep, shutdown_signal()).await
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
    tracing::info!(target: "codetether_agent::a2a::spawn", "Shutdown signal received");
}
