//! Bus completion emission for auto_process.

use uuid::Uuid;

use crate::events::{RlmCompletion, RlmOutcome};
use crate::result::RlmResult;

use super::types::{CrateAutoProcessContext, LoopOutcome};

/// Emit completion event on the bus, if present.
pub(super) fn emit_bus(
    ctx: &CrateAutoProcessContext<'_>,
    result: &RlmResult,
    outcome: &LoopOutcome,
    trace_id: Uuid,
) {
    if let Some(ref bus) = ctx.bus {
        bus.emit_completion(RlmCompletion {
            trace_id,
            outcome: classify_outcome(outcome),
            iterations: result.stats.iterations,
            subcalls: result.stats.subcalls,
            input_tokens: result.stats.input_tokens,
            output_tokens: result.stats.output_tokens,
            elapsed_ms: result.stats.elapsed_ms,
            reason: outcome.last_error.clone(),
            root_model: ctx.model.clone(),
            subcall_model_used: ctx.subcall_model.clone(),
        });
    }
}

fn classify_outcome(o: &LoopOutcome) -> RlmOutcome {
    if o.aborted {
        return RlmOutcome::Aborted;
    }
    if o.last_error.is_some() {
        return RlmOutcome::Failed;
    }
    if o.final_answer.is_some() { RlmOutcome::Converged } else { RlmOutcome::Exhausted }
}
