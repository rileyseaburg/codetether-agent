//! RLM refinement of a relay handoff with bounded fallback text.
use crate::semantic::extract_semantic_handoff_from_rlm;
use codetether_autochat_core::AUTOCHAT_RLM_HANDOFF_QUERY;
use codetether_autochat_core::model_rotation::{
    ModelRegistry, resolve_provider_for_model_autochat,
};
use codetether_rlm::router_executor::RlmExecutor;

/// Refine output with RLM when a provider is available and compose a handoff.
/// # Arguments
/// * `task` - Original relay task.
/// * `from_agent` - Author of the incoming handoff.
/// * `output` - Full output supplied to RLM.
/// * `model_ref` - Provider/model reference to resolve.
/// * `registry` - Optional initialized model registry.
/// # Returns
/// The composed handoff and whether a nonempty RLM result replaced its fallback.
/// # Examples
/// ```no_run
/// # async fn example<R: codetether_autochat_core::model_rotation::ModelRegistry>(registry: &std::sync::Arc<R>) {
/// let (message, used_rlm) = codetether_relay_rlm::prepare_autochat_handoff_with_registry("task", "worker", "output", "provider/model", Some(registry)).await;
/// # }
/// ```
pub async fn prepare_autochat_handoff_with_registry<R: ModelRegistry>(
    task: &str,
    from_agent: &str,
    output: &str,
    model_ref: &str,
    registry: Option<&std::sync::Arc<R>>,
) -> (String, bool) {
    let mut used_rlm = false;
    let mut relay_payload = crate::payload::fallback(output);

    if let Some(registry) = registry
        && let Some((provider, model_name)) =
            resolve_provider_for_model_autochat(registry.as_ref(), model_ref)
    {
        let mut executor =
            RlmExecutor::new(output.to_string(), provider, model_name).with_max_iterations(2);
        match executor.analyze(AUTOCHAT_RLM_HANDOFF_QUERY).await {
            Ok(result) => {
                let normalized = extract_semantic_handoff_from_rlm(&result.answer);
                if !normalized.is_empty() {
                    relay_payload = normalized;
                    used_rlm = true;
                }
            }
            Err(err) => {
                tracing::warn!(
                    error = %err,
                    "CLI RLM handoff normalization failed; using fallback payload"
                );
            }
        }
    }

    (
        crate::payload::message(task, from_agent, &relay_payload),
        used_rlm,
    )
}
