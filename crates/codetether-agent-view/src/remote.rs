//! Project observed peer state into the shared agent view.
use crate::{AgentOrigin, AgentSnapshot};

pub(crate) fn remote(parent_session_id: Option<&str>) -> Vec<AgentSnapshot> {
    codetether_peer_agent::observation::snapshots(parent_session_id)
        .into_iter()
        .map(|peer| AgentSnapshot {
            id: peer.name.clone(),
            name: peer.name,
            instructions: "Authenticated A2A LAN peer".to_string(),
            message_count: peer.message_count,
            depth: 0,
            is_processing: peer.is_processing,
            origin: AgentOrigin::lan_peer(),
            failed: peer.failed,
        })
        .collect()
}
