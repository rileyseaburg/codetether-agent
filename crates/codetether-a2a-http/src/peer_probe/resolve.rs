//! Readiness retries for advertised peers.
use super::*;

/// Resolve an advertised peer with bounded retries while it becomes ready.
///
/// # Arguments
/// * `peer` - Advertised peer and ordered candidate URLs.
/// * `self_url` - Local URL excluded from probing.
/// * `agent_name` - Local agent name used in diagnostic fields.
/// # Returns
/// The first successful endpoint and card, or `None` after five attempts.
/// Each candidate has a three-second timeout; retry delays grow from 200 ms.
/// # Examples
/// ```rust,no_run
/// # async fn example(peer: &codetether_a2a_discovery::mdns::DiscoveredPeer) {
/// use codetether_a2a_http::peer_probe::resolve;
/// let resolved = resolve(peer, "http://localhost:4096", "local-agent").await;
/// if let Some((endpoint, _)) = resolved { assert!(!endpoint.is_empty()); }
/// # }
/// ```
pub async fn resolve(
    peer: &DiscoveredPeer,
    self_url: &str,
    agent_name: &str,
) -> Option<(String, AgentCard)> {
    for attempt in 0..ATTEMPTS {
        if let Some(resolved) = resolve_once(peer, self_url, agent_name).await {
            return Some(resolved);
        }
        if attempt + 1 < ATTEMPTS {
            let delay = Duration::from_millis(200 * 2_u64.pow(attempt));
            tracing::debug!(target: "codetether_agent::a2a::spawn::mdns_probe", agent = %agent_name, peer = %peer.instance_name, ?delay,
                "mDNS peer advertised before it was ready; retrying");
            tokio::time::sleep(delay).await;
        }
    }
    None
}
