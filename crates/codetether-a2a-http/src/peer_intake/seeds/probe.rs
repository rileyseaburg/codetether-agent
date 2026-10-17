//! Sequential candidate probing for one configured seed.
use super::*;

pub(super) async fn probe(peer_seed: &str, agent_name: &str) -> Option<(String, AgentCard)> {
    let candidates = peer_candidates(peer_seed);
    let mut discovered_card = None;

    for candidate in candidates {
        match try_fetch_agent_card(&candidate).await {
            Ok(card) => {
                discovered_card = Some((candidate, card));
                break;
            }
            Err(error) => {
                tracing::debug!(target: "codetether_agent::a2a::spawn",
                    agent = %agent_name,
                    peer = %candidate,
                    error = %error,
                    "Peer probe failed"
                );
            }
        }
    }
    discovered_card
}
