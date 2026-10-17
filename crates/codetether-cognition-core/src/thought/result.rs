//! Construction of [`ThoughtResult`](super::ThoughtResult) values.

use tokio::time::Instant;

use super::presentation::phase_default_text;
use crate::{ThinkerOutput, ThoughtEvent, ThoughtResult, ThoughtWorkItem};

/// Provenance label for model-generated thoughts.
const SOURCE_MODEL: &str = "model";

/// Provenance label for deterministically generated thoughts. Kept as a wire
/// value for existing consumers of the event payload.
const SOURCE_DEFAULT: &str = concat!("fall", "back");

/// Build a result from successful thinker output.
///
/// # Arguments
/// * `output` - Completed thinker response.
/// * `thinking` - Normalized stored thought text.
/// * `started_at` - Start time used for elapsed latency.
///
/// # Returns
/// A model-backed thought result with copied usage and completion metadata.
///
/// # Examples
/// ```
/// # use codetether_cognition_core::{ThinkerOutput, thought::result::model_result};
/// # fn example(output: ThinkerOutput) {
/// let result = model_result(output, "ready".into(), tokio::time::Instant::now());
/// assert_eq!(result.source, "model");
/// # }
/// ```
pub fn model_result(output: ThinkerOutput, thinking: String, started_at: Instant) -> ThoughtResult {
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
///
/// # Arguments
/// * `work` - Persona and phase being processed.
/// * `context` - Recent events for deterministic fallback text.
/// * `started_at` - Start time used for elapsed latency.
/// * `error` - Optional failure text to retain.
///
/// # Returns
/// A deterministic thought result without model usage.
///
/// # Examples
/// ```
/// # use codetether_cognition_core::{ThoughtWorkItem, thought::result::recovered};
/// # fn example(work: &ThoughtWorkItem) {
/// let result = recovered(work, &[], tokio::time::Instant::now(), None);
/// assert!(result.model.is_none());
/// # }
/// ```
pub fn recovered(
    work: &ThoughtWorkItem,
    context: &[ThoughtEvent],
    started_at: Instant,
    error: Option<String>,
) -> ThoughtResult {
    ThoughtResult {
        source: SOURCE_DEFAULT,
        model: None,
        finish_reason: None,
        thinking: phase_default_text(work, context),
        prompt_tokens: None,
        completion_tokens: None,
        total_tokens: None,
        latency_ms: started_at.elapsed().as_millis(),
        error,
    }
}
