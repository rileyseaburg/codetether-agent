//! Construction of [`ThoughtResult`](super::ThoughtResult) values.

use tokio::time::Instant;

use super::{ThinkerOutput, ThoughtEvent, ThoughtResult, ThoughtWorkItem, phase_default};

/// Provenance label for model-generated thoughts.
const SOURCE_MODEL: &str = "model";

/// Provenance label for deterministically generated thoughts. Kept as a wire
/// value for existing consumers of the event payload.
const SOURCE_DEFAULT: &str = concat!("fall", "back");

/// Build a result from successful thinker output.
pub(super) fn model_result(
    output: ThinkerOutput,
    thinking: String,
    started_at: Instant,
) -> ThoughtResult {
    ThoughtResult {
        source: SOURCE_MODEL,
        model: Some(output.model),
        finish_reason: output.finish_reason,
        thinking,
        prompt_tokens: output.prompt_tokens,
        completion_tokens: output.completion_tokens,
        total_tokens: output.total_tokens,
        latency_ms: started_at.elapsed().as_millis(),
        error: None,
    }
}

/// Build a result backed by deterministic phase text.
pub(super) fn recovered(
    work: &ThoughtWorkItem,
    context: &[ThoughtEvent],
    started_at: Instant,
    error: Option<String>,
) -> ThoughtResult {
    ThoughtResult {
        source: SOURCE_DEFAULT,
        model: None,
        finish_reason: None,
        thinking: phase_default::phase_default_text(work, context),
        prompt_tokens: None,
        completion_tokens: None,
        total_tokens: None,
        latency_ms: started_at.elapsed().as_millis(),
        error,
    }
}
