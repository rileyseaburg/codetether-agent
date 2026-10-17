//! Execute one model request and record a failure in the loop outcome.

use crate::{
    router::types::{CrateAutoProcessContext, LoopOutcome},
    traits::{LlmMessage, LlmResponse, ToolDefinition},
};
use tracing::warn;
pub(super) async fn next(
    ctx: &CrateAutoProcessContext<'_>,
    state: &mut LoopOutcome,
    conversation: &[LlmMessage],
    tools: &[ToolDefinition],
) -> Option<LlmResponse> {
    let iterations = state.iterations;
    let (provider, model) = super::support::active_provider(ctx, iterations);
    match provider
        .complete(conversation.to_vec(), tools.to_vec(), &model, Some(0.7))
        .await
    {
        Ok(response) => Some(response),
        Err(e) => {
            warn!(error = %e, iteration = iterations, "RLM: Model call failed");
            state.last_error = Some(format!("Model call failed at iteration {iterations}: {e}"));
            None
        }
    }
}
