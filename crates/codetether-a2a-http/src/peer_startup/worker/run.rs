use super::{
    environment::{worker_a2a_mdns_enabled, worker_a2a_peer_seeds, worker_a2a_port},
    url::worker_public_url_for_port,
};
use crate::{peer_handle::A2APeerHandle, peer_startup::SpawnOptions};
use std::future::Future;
/// Start the optional worker-local peer with environment-selected discovery.
///
/// # Arguments
/// * `hostname` — Listener bind host.
/// * `public_url` — Optional worker public URL, adjusted to the peer port.
/// * `name` — Base worker identity used to name and describe the peer.
/// * `start` — Native endpoint startup callback receiving the complete options.
/// # Returns
/// The started peer handle, or None when disabled or startup fails. Startup
/// failures are logged and leave the worker's SSE path available.
/// # Examples
/// ```ignore
/// let peer = maybe_start_worker_a2a_peer(hostname, public_url, name, start).await;
/// ```
pub async fn maybe_start_worker_a2a_peer<F, Fut>(
    hostname: &str,
    public_url: Option<&str>,
    name: &str,
    start: F,
) -> Option<A2APeerHandle>
where
    F: FnOnce(SpawnOptions) -> Fut,
    Fut: Future<Output = anyhow::Result<A2APeerHandle>>,
{
    if !worker_a2a_mdns_enabled() {
        tracing::info!(
            "Worker A2A mDNS peer disabled; set CODETETHER_WORKER_A2A_MDNS=true to enable"
        );
        return None;
    }
    let peer = worker_a2a_peer_seeds();
    if !peer.is_empty() {
        tracing::info!(peer_count = peer.len(), "Worker A2A peer seeds configured");
    }
    let opts = SpawnOptions {
        name: Some(format!("{name}-a2a")),
        hostname: hostname.to_owned(),
        port: worker_a2a_port(),
        public_url: worker_public_url_for_port(public_url, worker_a2a_port()),
        description: Some(format!("CodeTether harvester worker A2A peer for {name}")),
        peer,
        discovery_interval_secs: 15,
        auto_introduce: true,
        mdns: true,
    };
    match start(opts).await {
        Ok(handle) => {
            tracing::info!(agent = %handle.agent_name, public_url = %handle.public_url, "Worker A2A mDNS peer started");
            Some(handle)
        }
        Err(error) => {
            tracing::warn!(%error, "Worker A2A mDNS peer failed to start; continuing SSE worker path");
            None
        }
    }
}
