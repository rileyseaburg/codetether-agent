//! profiles planning operation using the shared provider registry contract.
use codetether_autochat_core::AUTOCHAT_MAX_AGENTS;
use codetether_autochat_core::model_rotation::{
    ModelRegistry, resolve_provider_for_model_autochat,
};
use codetether_relay_plan::{
    PlannedRelayResponse, RelayProfile, build_runtime_profile_from_plan, extract_json_payload,
};

/// Ask for a bounded team and normalize its specialist profiles.
/// # Arguments
/// * `task` - User task to plan.
/// * `model_ref` - Requested provider/model reference.
/// * `requested_agents` - Requested team size, clamped to the supported range.
/// * `registry` - Existing initialized model registry.
/// # Returns
/// At least two resolved specialists, or none when planning cannot supply a team.
/// # Examples
/// ```no_run
/// # async fn example<R: codetether_relay_planner::ModelRegistry>(registry: &std::sync::Arc<R>) {
/// let planned = codetether_relay_planner::plan_relay_profiles_with_registry("repair", "provider/model", 3, registry).await;
/// # }
/// ```
pub async fn plan_relay_profiles_with_registry<R: ModelRegistry>(
    task: &str,
    model_ref: &str,
    requested_agents: usize,
    registry: &std::sync::Arc<R>,
) -> Option<Vec<RelayProfile>> {
    let (provider, model_name) = resolve_provider_for_model_autochat(registry.as_ref(), model_ref)?;
    let requested_agents = requested_agents.clamp(2, AUTOCHAT_MAX_AGENTS);

    let request = crate::profiles_request::build(model_name, task, requested_agents);

    let response = provider.complete(request).await.ok()?;
    let text = crate::text::extract(&response.message.content);

    let planned = extract_json_payload::<PlannedRelayResponse>(&text)?;
    let mut existing = Vec::<String>::new();
    let mut runtime = Vec::<RelayProfile>::new();

    for profile in planned.profiles.into_iter().take(AUTOCHAT_MAX_AGENTS) {
        if let Some(runtime_profile) = build_runtime_profile_from_plan(profile, &existing) {
            existing.push(runtime_profile.name.clone());
            runtime.push(runtime_profile);
        }
    }

    if runtime.len() >= 2 {
        Some(runtime)
    } else {
        None
    }
}
