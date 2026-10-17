//! One pass through advertised endpoint candidates.
use super::*;

pub(super) async fn resolve_once(
    peer: &DiscoveredPeer,
    self_url: &str,
    agent_name: &str,
) -> Option<(String, AgentCard)> {
    for url in &peer.urls {
        if url.trim_end_matches('/') == self_url {
            continue;
        }
        for candidate in peer_candidates(url) {
            match tokio::time::timeout(PROBE_TIMEOUT, try_fetch_agent_card(&candidate)).await {
                Ok(Ok(card)) => return Some((candidate, card)),
                Ok(Err(error)) => {
                    tracing::debug!(target: "codetether_agent::a2a::spawn::mdns_probe", agent = %agent_name, peer = %candidate,
                    %error, "mDNS peer probe failed")
                }
                Err(_) => {
                    tracing::debug!(target: "codetether_agent::a2a::spawn::mdns_probe", agent = %agent_name, peer = %candidate,
                    "mDNS peer probe timed out")
                }
            }
        }
    }
    None
}
