//! Readiness-tolerant probing for peers resolved through mDNS.

use super::{peer_probe::try_fetch_agent_card, peer_url::peer_candidates};
use crate::a2a::{mdns::DiscoveredPeer, types::AgentCard};
use std::time::Duration;

const ATTEMPTS: u32 = 5;
const PROBE_TIMEOUT: Duration = Duration::from_secs(3);

pub(super) async fn resolve(
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
            tracing::debug!(agent = %agent_name, peer = %peer.instance_name, ?delay,
                "mDNS peer advertised before it was ready; retrying");
            tokio::time::sleep(delay).await;
        }
    }
    None
}

async fn resolve_once(
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
                Ok(Err(error)) => tracing::debug!(agent = %agent_name, peer = %candidate,
                    %error, "mDNS peer probe failed"),
                Err(_) => tracing::debug!(agent = %agent_name, peer = %candidate,
                    "mDNS peer probe timed out"),
            }
        }
    }
    None
}
