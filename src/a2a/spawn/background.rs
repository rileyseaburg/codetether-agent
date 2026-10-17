//! Peer startup background.
use super::*;

/// Start an A2A peer runtime in the background, attached to the given bus.
///
/// Caller is responsible for keeping the returned [`A2APeerHandle`] alive
/// for as long as the peer should keep serving. Drop or call `abort()` on
/// shutdown. Used by the TUI on every launch (unless `--no-a2a` was set).
pub async fn start_a2a_in_background(
    opts: SpawnOptions,
    bus: Arc<AgentBus>,
) -> Result<A2APeerHandle> {
    let prep = prepare_a2a(opts, bus).await?;
    tracing::info!(target: "codetether_agent::a2a::spawn",
        agent = %prep.agent_name,
        bind_addr = %prep.bind_addr,
        public_url = %prep.public_url,
        mdns = prep.mdns_handle.is_some(),
        "A2A peer listening (background mode)"
    );
    let agent_name = prep.agent_name.clone();
    local_identity::activate(&agent_name);
    let bind_addr = prep.bind_addr.clone();
    let public_url = prep.public_url.clone();
    let server_task = tokio::spawn(async move {
        if let Err(e) = axum::serve(prep.listener, prep.router).await {
            tracing::error!(target: "codetether_agent::a2a::spawn", error = %e, "A2A peer server task exited with error");
        }
    });
    Ok(A2APeerHandle::new(
        agent_name,
        bind_addr,
        public_url,
        server_task,
        prep.discovery_task,
        prep.mdns_intake_task,
        prep.lan_tasks,
        prep.mdns_handle,
    ))
}
