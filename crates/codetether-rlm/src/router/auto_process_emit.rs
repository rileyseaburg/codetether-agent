//! Construct the final RLM result and completion log.

mod measure;
use super::types::LoopOutcome;
use crate::{result::RlmResult, stats::RlmStats};
use std::time::Instant;
use tracing::info;
use uuid::Uuid;

/// Build the final `RlmResult`.
pub(super) fn build_result(
    answer: &str,
    input_tokens: usize,
    outcome: &LoopOutcome,
    start: Instant,
    trace_id: Uuid,
) -> RlmResult {
    let (output_tokens, elapsed, ratio) = measure::measure(answer, input_tokens, start);
    let processed = format!(
        "[RLM: {input_tokens} → {output_tokens} tokens | {} iterations | {} sub-calls]\n\n{}",
        outcome.iterations, outcome.subcalls, answer
    );
    let success = !outcome.aborted && outcome.final_answer.is_some();
    let error = outcome
        .aborted
        .then_some("RLM loop aborted by caller".into())
        .or(outcome.last_error.clone());
    info!(
        input_tokens,
        output_tokens,
        iterations = outcome.iterations,
        subcalls = outcome.subcalls,
        elapsed,
        "RLM: Processing complete"
    );
    RlmResult {
        processed,
        success,
        error,
        stats: RlmStats {
            input_tokens,
            output_tokens,
            iterations: outcome.iterations,
            subcalls: outcome.subcalls,
            elapsed_ms: elapsed,
            compression_ratio: ratio,
        },
        trace: None,
        trace_id: Some(trace_id),
    }
}
