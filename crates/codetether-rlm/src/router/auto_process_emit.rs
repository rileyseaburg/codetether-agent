//! Result construction and bus emission for auto_process.

use std::time::Instant;
use tracing::info;
use uuid::Uuid;

use crate::chunker::RlmChunker;
use crate::events::{RlmCompletion, RlmOutcome};
use crate::result::RlmResult;
use crate::stats::RlmStats;

use super::auto_loop;
use super::types::CrateAutoProcessContext;

/// Build the final `RlmResult`.
pub(super) fn build_result(
    answer: &str, input_tokens: usize, outcome: &auto_loop::LoopOutcome,
    start: Instant, trace_id: Uuid,
) -> RlmResult {
    let output_tokens = RlmChunker::estimate_tokens(answer);
    let elapsed = start.elapsed().as_millis() as u64;
    let ratio = input_tokens as f64 / output_tokens.max(1) as f64;
    let processed = format!(
        "[RLM: {input_tokens} → {output_tokens} tokens | {} iterations | {} sub-calls]\n\n{}",
        outcome.iterations, outcome.subcalls, answer
    );
    let success = !outcome.aborted && outcome.final_answer.is_some();
    info!(input_tokens, output_tokens, iterations = outcome.iterations, subcalls = outcome.subcalls, elapsed, "RLM: Processing complete");
    RlmResult {
        processed, success, error: None,
        stats: RlmStats { input_tokens, output_tokens, iterations: outcome.iterations, subcalls: outcome.subcalls, elapsed_ms: elapsed, compression_ratio: ratio },
        trace: None, trace_id: Some(trace_id),
    }
}

/// Emit completion event on the bus, if present.
pub(super) fn emit_bus(
    ctx: &CrateAutoProcessContext<'_>, result: &RlmResult,
    outcome: &auto_loop::LoopOutcome, trace_id: Uuid,
) {
    if let Some(ref bus) = ctx.bus {
        let rlm_outcome = if outcome.aborted { RlmOutcome::Aborted }
            else if outcome.final_answer.is_some() { RlmOutcome::Converged }
            else { RlmOutcome::Exhausted };
        bus.emit_completion(RlmCompletion {
            trace_id, outcome: rlm_outcome, iterations: result.stats.iterations,
            subcalls: result.stats.subcalls, input_tokens: result.stats.input_tokens,
            output_tokens: result.stats.output_tokens, elapsed_ms: result.stats.elapsed_ms,
            reason: None, root_model: ctx.model.clone(),
            subcall_model_used: ctx.subcall_model.clone(),
        });
    }
}
