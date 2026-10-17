//! Serve a prepared peer endpoint until application-requested shutdown.
use super::*;
use std::future::Future;

/// Serve an endpoint and stop its discovery tasks after graceful shutdown.
///
/// # Arguments
///
/// * `prep` - Prepared listener, router and discovery task ownership.
/// * `shutdown` - Application signal that resolves when serving should stop.
///
/// # Returns
///
/// Returns after serving stops and discovery tasks are aborted.
///
/// # Errors
///
/// Returns the server error if serving fails.
///
/// # Examples
///
/// ```rust,no_run
/// # async fn demo(prep: codetether_a2a_http::peer_startup::A2APreparation) -> anyhow::Result<()> {
/// codetether_a2a_http::peer_startup::serve_foreground(prep, async {}).await
/// # }
/// ```
pub async fn serve_foreground(
    prep: A2APreparation,
    shutdown: impl Future<Output = ()> + Send + 'static,
) -> Result<()> {
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
        .with_graceful_shutdown(shutdown)
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
