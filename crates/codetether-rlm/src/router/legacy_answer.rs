//! Final answer selection for the legacy loop.

use tracing::warn;

use super::types::{CrateAutoProcessContext, LoopOutcome};

/// Return the loop answer or a structured fallback excerpt.
pub fn choose(
    output: &str,
    ctx: &CrateAutoProcessContext<'_>,
    input_tokens: usize,
    outcome: &LoopOutcome,
) -> String {
    outcome.final_answer.clone().unwrap_or_else(|| {
        warn!(
            iterations = outcome.iterations,
            "RLM: No FINAL produced, using fallback"
        );
        super::fallback::enhanced_fallback(output, ctx.tool_id, &ctx.tool_args, input_tokens)
    })
}
