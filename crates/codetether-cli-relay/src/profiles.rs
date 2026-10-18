//! Model-planned initial team with the original fallback.
use crate::Host;
use codetether_relay_plan::{RelayProfile, build_relay_profiles};
use codetether_relay_planner::plan_relay_profiles_with_registry;
use std::sync::Arc;
pub(super) async fn select<H: Host>(
    registry: &Option<Arc<H::Registry>>,
    task: &str,
    model_ref: &str,
    agent_count: usize,
) -> (Vec<RelayProfile>, bool) {
    let mut planner_used = false;
    let profiles = if let Some(registry) = registry {
        if let Some(planned) =
            plan_relay_profiles_with_registry(task, model_ref, agent_count, registry).await
        {
            planner_used = true;
            planned
        } else {
            build_relay_profiles(agent_count)
        }
    } else {
        build_relay_profiles(agent_count)
    };

    (profiles, planner_used)
}
