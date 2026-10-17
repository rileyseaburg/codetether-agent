//! Runtime shutdown without changing descendant spawn-edge lifecycle.

use crate::Host;
use anyhow::Result;
use codetether_collaboration_state::{execution as execution_state, residency, thread_status};
use std::collections::{HashSet, VecDeque};

/// Shut down one resident runtime without changing its durable edge status.
pub(super) async fn one<H: Host>(agent_id: &str) {
    if H::get(agent_id).is_none() {
        return;
    }
    execution_state::close(agent_id);
    thread_status::shutdown(agent_id);
    if !super::wait::until_idle(agent_id).await {
        tracing::warn!(
            target: "codetether_agent::tool::agent::thread_lifecycle::shutdown",
            agent_id,
            "Child shutdown did not settle; keeping runtime resident"
        );
        return;
    }
    H::remove_runtime(agent_id);
}

/// Shut down every durable descendant that currently has a resident runtime.
///
/// # Errors
///
/// Returns an error when durable child-edge discovery fails.
pub(super) async fn descendants<H: Host>(root_id: &str) -> Result<()> {
    let mut queue = VecDeque::from([root_id.to_string()]);
    let mut seen = HashSet::from([root_id.to_string()]);
    let mut transitions = Vec::new();
    while let Some(parent_id) = queue.pop_front() {
        for child_id in H::child_ids(&parent_id).await? {
            if !seen.insert(child_id.clone()) {
                continue;
            }
            let transition = residency::acquire(&child_id).await;
            one::<H>(&child_id).await;
            queue.push_back(child_id);
            transitions.push(transition);
        }
    }
    Ok(())
}
