//! Project resident child snapshots without changing registry order.
use crate::{AgentOrigin, AgentSnapshot, Host};
use codetether_agent_registry::Session;

pub(crate) fn local<H: Host>(parent_session_id: Option<&str>) -> Vec<AgentSnapshot> {
    H::entries(parent_session_id)
        .into_iter()
        .map(|entry| AgentSnapshot {
            id: entry.session.id().to_owned(),
            name: entry.name,
            instructions: entry.instructions,
            message_count: H::message_count(&entry.session),
            depth: entry.depth,
            is_processing: codetether_collaboration_state::execution::is_running(
                entry.session.id(),
            ),
            origin: AgentOrigin::Local {
                parent: entry.parent,
                model_id: entry.model_id,
            },
            failed: false,
        })
        .collect()
}
