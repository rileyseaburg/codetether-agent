//! Worker-side A2A peer startup helpers.

use std::sync::Arc;

use crate::a2a::spawn::{A2APeerHandle, SpawnOptions};
use crate::bus::AgentBus;
use crate::cli::A2aArgs;

use super::a2a_peer_config::{
    worker_a2a_mdns_enabled, worker_a2a_peer_seeds, worker_a2a_port, worker_public_url_for_port,
};

/// Starts the optional worker-local A2A peer used for mDNS discovery.
pub(super) async fn maybe_start_worker_a2a_peer(
    args: &A2aArgs,
    name: &str,
    bus: Arc<AgentBus>,
) -> Option<A2APeerHandle> {
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
        hostname: args.hostname.clone(),
        port: worker_a2a_port(),
        public_url: worker_public_url_for_port(args, worker_a2a_port()),
        description: Some(format!("CodeTether harvester worker A2A peer for {name}")),
        peer,
        discovery_interval_secs: 15,
        auto_introduce: true,
        mdns: true,
    };
    match crate::a2a::spawn::start_a2a_in_background(opts, bus).await {
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
