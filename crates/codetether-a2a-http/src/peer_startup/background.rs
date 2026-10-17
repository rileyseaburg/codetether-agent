//! Run a prepared peer endpoint in a background task.
use super::*;
use crate::peer_handle::A2APeerHandle;
use codetether_a2a_discovery::local_identity;

/// Start serving a prepared endpoint and activate its local identity.
///
/// # Arguments
///
/// * `prep` - Bound endpoint and running discovery tasks to take ownership of.
///
/// # Returns
///
/// A handle that stops server and discovery tasks when dropped. Server errors
/// are logged by the spawned task.
///
/// # Examples
///
/// ```rust
/// use codetether_a2a_http::peer_startup::start_background;
/// let _start = start_background;
/// ```
pub fn start_background(prep: A2APreparation) -> A2APeerHandle {
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
    A2APeerHandle::new(
        agent_name,
        bind_addr,
        public_url,
        server_task,
        prep.discovery_task,
        prep.mdns_intake_task,
        prep.lan_tasks,
        prep.mdns_handle,
    )
}
