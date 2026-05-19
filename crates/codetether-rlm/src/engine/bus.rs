//! Session bus emission for engine runs.

use uuid::Uuid;

use crate::events::{RlmCompletion, RlmOutcome};
use crate::result::RlmResult;
use crate::router::CrateAutoProcessContext;

/// Emit an engine completion event when a bus is available.
pub fn emit(ctx: &CrateAutoProcessContext<'_>, result: &RlmResult, trace_id: Uuid) {
    if let Some(ref bus) = ctx.bus {
        bus.emit_completion(RlmCompletion {
            trace_id,
            outcome: RlmOutcome::Converged,
            iterations: result.stats.iterations,
            subcalls: result.stats.subcalls,
            input_tokens: result.stats.input_tokens,
            output_tokens: result.stats.output_tokens,
            elapsed_ms: result.stats.elapsed_ms,
            reason: None,
            root_model: ctx.model.clone(),
            subcall_model_used: None,
        });
    }
}
