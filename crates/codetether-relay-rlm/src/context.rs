//! RLM execution adapter over shared relay context policy.
use codetether_autochat_core::shared_context::{
    ContextDelta, context_distillation_query, extract_context_delta_from_text,
    fallback_context_delta,
};
use codetether_provider_core::Provider;
use codetether_rlm::router_executor::RlmExecutor;
use std::sync::Arc;

/// Distill shared context from output, retaining the deterministic fallback.
/// # Arguments
/// * `output` - Full worker output.
/// * `task` - Original relay task.
/// * `from_agent` - Author of the output.
/// * `provider_and_model` - Optional provider and resolved model.
/// # Returns
/// The parsed or fallback delta and whether RLM supplied it.
/// # Examples
/// ```no_run
/// # async fn example() {
/// let (_, used_rlm) = codetether_relay_rlm::distill_context_delta_with_rlm("output", "task", "worker", None).await;
/// assert!(!used_rlm);
/// # }
/// ```
pub async fn distill_context_delta_with_rlm(
    output: &str,
    task: &str,
    from_agent: &str,
    provider_and_model: Option<(Arc<dyn Provider>, String)>,
) -> (ContextDelta, bool) {
    if let Some((provider, model_name)) = provider_and_model {
        let query = context_distillation_query(task, from_agent);
        let mut executor =
            RlmExecutor::new(output.to_string(), provider, model_name).with_max_iterations(2);
        match executor.analyze(&query).await {
            Ok(result) => {
                if let Some(delta) = extract_context_delta_from_text(&result.answer) {
                    return (delta, true);
                }
            }
            Err(err) => {
                tracing::warn!(
                    error = %err,
                    agent = %from_agent,
                    "RLM context distillation failed; using fallback delta"
                );
            }
        }
    }

    (fallback_context_delta(output, from_agent), false)
}
