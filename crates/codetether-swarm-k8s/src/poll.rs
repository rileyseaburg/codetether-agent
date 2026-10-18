//! Polling pass across all active Kubernetes sub-agents.

use super::{poll_one, state::State};
use codetether_swarm_core::SubTaskResult;

pub(super) async fn finished(state: &mut State<'_, impl crate::Host>) -> Vec<SubTaskResult> {
    let active = state
        .active
        .iter()
        .map(|(id, branch)| (id.clone(), branch.clone()))
        .collect::<Vec<_>>();
    let mut results = Vec::new();
    for (id, branch) in active {
        if let Some(result) = poll_one::execute(state, &id, &branch).await {
            results.push(result);
        }
    }
    results
}
