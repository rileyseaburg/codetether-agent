//! Result construction and bus emission for auto_process.

use std::time::Instant;
use tracing::info;
use uuid::Uuid;

use crate::chunker::RlmChunker;
use crate::result::RlmResult;
use crate::stats::RlmStats;

use super::types::LoopOutcome;

/// Build the final `RlmResult`.
pub(super) fn build_result(answer: &str, input_tokens: usize, outcome: &LoopOutcome,
    start: Instant, trace_id: Uuid) -> RlmResult {
    let output_tokens = RlmChunker::estimate_tokens(answer);
    let elapsed = start.elapsed().as_millis() as u64;
    let ratio = if output_tokens == 0 { 0.0 } else { output_tokens as f64 / input_tokens as f64 };
    let processed = format!(
        "[RLM: {input_tokens} → {output_tokens} tokens | {} iterations | {} sub-calls]\n\n{}",
        outcome.iterations, outcome.subcalls, answer
    );
    let success = !outcome.aborted && outcome.final_answer.is_some();
    let error = outcome.aborted.then_some("RLM loop aborted by caller".into()).or(outcome.last_error.clone());
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
        stats: RlmStats { input_tokens, output_tokens, iterations: outcome.iterations, subcalls: outcome.subcalls, elapsed_ms: elapsed, compression_ratio: ratio },
        trace: None,
        trace_id: Some(trace_id),
    }
}
