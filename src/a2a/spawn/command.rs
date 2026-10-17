//! Peer startup command.
use super::*;

/// Run the `codetether spawn` command. Owns its own bus and S3 sink, blocks
/// on ctrl_c with graceful shutdown.
pub async fn run(args: SpawnArgs) -> Result<()> {
    let bus = AgentBus::new().into_arc();

    // Auto-start S3 sink if MinIO is configured
    crate::bus::s3_sink::spawn_bus_s3_sink(bus.clone());

    let prep = prepare_a2a(SpawnOptions::from_spawn_args(&args), bus).await?;

    tracing::info!(target: "codetether_agent::a2a::spawn",
        agent = %prep.agent_name,
        bind_addr = %prep.bind_addr,
        public_url = %prep.public_url,
        mdns = prep.mdns_handle.is_some(),
        "Spawned A2A agent runtime"
    );

    let agent_name = prep.agent_name.clone();
    let discovery_task = prep.discovery_task;
    let mdns_intake_task = prep.mdns_intake_task;
    let lan_tasks = prep.lan_tasks;
    let _mdns_handle = prep.mdns_handle; // dropped at end of fn → unregisters

    axum::serve(prep.listener, prep.router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("Spawned A2A server failed")?;

    discovery_task.abort();
    if let Some(t) = mdns_intake_task {
        t.abort();
    }
    for task in lan_tasks {
        task.abort();
    }
    tracing::info!(target: "codetether_agent::a2a::spawn", agent = %agent_name, "Spawned A2A agent shut down");
    Ok(())
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
    tracing::info!(target: "codetether_agent::a2a::spawn", "Shutdown signal received");
}
