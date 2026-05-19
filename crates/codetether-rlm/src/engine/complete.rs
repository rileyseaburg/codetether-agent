//! Shared completion helper for engine runs.

use std::time::Instant;
use uuid::Uuid;

use crate::result::RlmResult;
use crate::router::CrateAutoProcessContext;

use super::{bus, evidence::Evidence, response};

/// Build the final result and emit completion.
pub fn finish(
    ctx: &CrateAutoProcessContext<'_>,
    answer: String,
    input_tokens: usize,
    start: Instant,
    trace_id: Uuid,
    evidence: Option<&Evidence>,
) -> RlmResult {
    let result = response::success(answer, input_tokens, start, trace_id, evidence);
    bus::emit(ctx, &result, trace_id);
    result
}
