//! okr planning operation using the shared provider registry contract.
use codetether_autochat_core::model_rotation::{
    ModelRegistry, resolve_provider_for_model_autochat,
};
use codetether_relay_plan::{PlannedOkrDraft, extract_json_payload};

/// Ask the selected provider for an execution-governance objective.
/// # Arguments
/// * `task` - User task to plan.
/// * `model_ref` - Requested provider/model reference.
/// * `agent_count` - Team size included in the planning prompt.
/// * `registry` - Existing initialized model registry.
/// # Returns
/// A parsed objective draft, or none for unavailable providers, completion errors or invalid JSON.
/// # Examples
/// ```no_run
/// # async fn example<R: codetether_relay_planner::ModelRegistry>(registry: &std::sync::Arc<R>) {
/// let planned = codetether_relay_planner::plan_okr_draft_with_registry("repair", "provider/model", 3, registry).await;
/// # }
/// ```
pub async fn plan_okr_draft_with_registry<R: ModelRegistry>(
    task: &str,
    model_ref: &str,
    agent_count: usize,
    registry: &std::sync::Arc<R>,
) -> Option<PlannedOkrDraft> {
    let (provider, model_name) = resolve_provider_for_model_autochat(registry.as_ref(), model_ref)?;
    let model_name_for_log = model_name.clone();

    // Keep prompt short-ish: this happens *before* the approval gate.
    let request = crate::okr_request::build(model_name, task, agent_count);

    let response = provider.complete(request).await.ok()?;
    let text = crate::text::extract(&response.message.content);

    tracing::debug!(
        model = %model_name_for_log,
        response_len = text.len(),
        response_preview = %text.chars().take(500).collect::<String>(),
        "OKR draft model response"
    );

    let parsed = extract_json_payload::<PlannedOkrDraft>(&text);
    if parsed.is_none() {
        tracing::warn!(
            model = %model_name_for_log,
            response_preview = %text.chars().take(500).collect::<String>(),
            "Failed to parse OKR draft JSON from model response"
        );
    }
    parsed
}
