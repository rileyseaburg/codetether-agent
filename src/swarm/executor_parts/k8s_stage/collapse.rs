//! Collapse-policy sampling across active Kubernetes branches.

use super::{collapse_audit, collapse_kill, observation, state::State};

pub(super) async fn sample(state: &mut State<'_>) {
    if !state.executor.config.collapse_enabled || state.active.len() <= 1 {
        return;
    }
    let active = state
        .active
        .iter()
        .map(|(id, item)| (id.clone(), item.clone()))
        .collect::<Vec<_>>();
    let mut observations = Vec::with_capacity(active.len());
    for (id, item) in active {
        observations.push(observation::build(state, &id, &item).await);
    }
    let tick = state.collapse.sample_observations(&observations);
    if state.promoted != tick.promoted_subtask_id {
        state.promoted = tick.promoted_subtask_id;
        if let Some(id) = &state.promoted {
            tracing::info!(subtask_id = %id, "Collapse controller promoted Kubernetes branch");
            collapse_audit::promotion(state.swarm_id, id).await;
        }
    }
    for kill in tick.kills {
        if let Some(result) = collapse_kill::apply(state, kill).await {
            state.results.push(result);
        }
    }
}
