//! RLM execution adapter over shared relay context policy.
use crate::provider::Provider;
use crate::rlm::RlmExecutor;
pub use codetether_autochat_core::shared_context::*;
use std::sync::Arc;

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
