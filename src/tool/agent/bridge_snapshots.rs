//! Unified local-child and remote-peer snapshots for TUI consumers.

use super::{AgentOrigin, AgentSnapshot, store};

pub(super) fn all(parent_session_id: Option<&str>) -> Vec<AgentSnapshot> {
    let mut agents = local(parent_session_id);
    agents.extend(remote(parent_session_id));
    agents
}

fn local(parent_session_id: Option<&str>) -> Vec<AgentSnapshot> {
    store::entries_for_parent(parent_session_id)
        .into_iter()
        .map(|entry| AgentSnapshot {
            id: entry.session.id.clone(),
            name: entry.name,
            instructions: entry.instructions,
            message_count: entry.session.messages.len(),
            depth: entry.depth,
            is_processing: crate::tool::agent::execution_state::is_running(&entry.session.id),
            origin: AgentOrigin::Local {
                parent: entry.parent,
                model_id: entry.model_id,
            },
            failed: false,
        })
        .collect()
}

fn remote(parent_session_id: Option<&str>) -> Vec<AgentSnapshot> {
    crate::tool::agent::message::remote::observation::snapshots(parent_session_id)
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
