//! Runtime shutdown without changing descendant spawn-edge lifecycle.

use super::super::{collaboration_runtime::thread_status, execution_state};
use super::super::{event_loop, persistence, residency, store};
use anyhow::Result;
use std::collections::{HashSet, VecDeque};

/// Shut down one resident runtime without changing its durable edge status.
pub(super) async fn one(agent_id: &str) {
    if store::get(agent_id).is_none() {
        return;
    }
    execution_state::close(agent_id);
    thread_status::shutdown(agent_id);
    if !super::wait::until_idle(agent_id).await {
        tracing::warn!(
            agent_id,
            "Child shutdown did not settle; keeping runtime resident"
        );
        return;
    }
    store::remove(agent_id);
    event_loop::live_trace::clear(agent_id);
}

/// Shut down every durable descendant that currently has a resident runtime.
///
/// # Errors
///
/// Returns an error when durable child-edge discovery fails.
pub(super) async fn descendants(root_id: &str) -> Result<()> {
    let mut queue = VecDeque::from([root_id.to_string()]);
    let mut seen = HashSet::from([root_id.to_string()]);
    let mut transitions = Vec::new();
    while let Some(parent_id) = queue.pop_front() {
        for child_id in persistence::child_ids(&parent_id).await? {
            if !seen.insert(child_id.clone()) {
                continue;
            }
            let transition = residency::acquire_transition(&child_id).await;
            one(&child_id).await;
            queue.push_back(child_id);
            transitions.push(transition);
        }
    }
    Ok(())
}
