//! Resolve, register and introduce a discovered peer.
use super::*;

/// Resolve a discovered peer, then register and optionally introduce it.
pub(super) async fn handle_mdns_peer(
    bus: Arc<AgentBus>,
    liveness: Arc<Mutex<PeerLiveness>>,
    self_url: Arc<String>,
    agent_name: String,
    peer: DiscoveredPeer,
    auto_introduce: bool,
    resolve_path: fn() -> Option<PathBuf>,
) {
    let Some((endpoint, card)) = mdns_probe::resolve(&peer, &self_url, &agent_name).await else {
        return;
    };

    let is_new = {
        let mut l = liveness.lock().await;
        l.record(&peer.instance_name, &card.name)
    };

    bus.registry.register(card.clone());
    routes::register(&card, &endpoint, peer.token.clone());

    // Re-emit on every accepted resolution (not just first sight) so a peer
    // that was registered once but later dropped by a consumer reappears on
    // the next mDNS re-resolution instead of vanishing permanently.
    presence::emit_discovered(&bus, &card.name, &endpoint);

    if is_new {
        tracing::info!(target: "codetether_agent::a2a::spawn",
            agent = %agent_name,
            peer_name = %card.name,
            peer_url = %card.url,
            endpoint = %endpoint,
            via = "mdns",
            "Discovered A2A peer"
        );
        if auto_introduce
            && let Err(error) = send_intro(
                &endpoint,
                &agent_name,
                self_url.as_str(),
                peer.token.as_deref(),
                resolve_path,
            )
            .await
        {
            tracing::warn!(target: "codetether_agent::a2a::spawn",
                agent = %agent_name,
                peer = %endpoint,
                error = %error,
                "Auto-intro message failed"
            );
        }
    }
}
