//! spawn planning operation using the shared provider registry contract.
use codetether_autochat_core::model_rotation::{
    ModelRegistry, resolve_provider_for_model_autochat,
};
use codetether_relay_plan::{
    RelayProfile, RelaySpawnDecision, build_runtime_profile_from_plan, extract_json_payload,
    truncate_with_ellipsis,
};

/// Ask whether the current relay needs an additional specialist.
/// # Arguments
/// * `task` - User task to plan.
/// * `model_ref` - Requested provider/model reference.
/// * `latest_output` - Latest handoff, truncated for the prompt.
/// * `round` - Current relay round.
/// * `ordered_agents` - Existing names used for team context and uniqueness.
/// * `registry` - Existing initialized model registry.
/// # Returns
/// A specialist and reason when requested and valid; otherwise none.
/// # Examples
/// ```no_run
/// # async fn example<R: codetether_relay_planner::ModelRegistry>(registry: &std::sync::Arc<R>) {
/// let planned = codetether_relay_planner::decide_dynamic_spawn_with_registry("repair", "provider/model", "handoff", 1, &[], registry).await;
/// # }
/// ```
pub async fn decide_dynamic_spawn_with_registry<R: ModelRegistry>(
    task: &str,
    model_ref: &str,
    latest_output: &str,
    round: usize,
    ordered_agents: &[String],
    registry: &std::sync::Arc<R>,
) -> Option<(RelayProfile, String)> {
    let (provider, model_name) = resolve_provider_for_model_autochat(registry.as_ref(), model_ref)?;
    let team = ordered_agents
        .iter()
        .map(|name| format!("@{name}"))
        .collect::<Vec<_>>()
        .join(", ");
    let output_excerpt = truncate_with_ellipsis(latest_output, 2200);

    let request = crate::spawn_request::build(model_name, task, &team, &output_excerpt, round);

    let response = provider.complete(request).await.ok()?;
    let text = crate::text::extract(&response.message.content);

    let decision = extract_json_payload::<RelaySpawnDecision>(&text)?;
    if !decision.spawn {
        return None;
    }

    let profile = decision.profile?;
    let runtime_profile = build_runtime_profile_from_plan(profile, ordered_agents)?;
    let reason = if decision.reason.trim().is_empty() {
        "Model requested additional specialist for task scope.".to_string()
    } else {
        decision.reason.trim().to_string()
    };

    Some((runtime_profile, reason))
}
