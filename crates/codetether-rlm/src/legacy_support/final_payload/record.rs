//! Append the existing normalization trace and final context event.
use super::*;
pub(super) const NORMALIZE: &str = "normalize_final_payload(grep_trace)";
pub(super) const COERCE: &str = "coerce_final_payload(grep_trace)";
pub(super) fn record_normalization(
    trace_steps: &mut Vec<TraceStep>,
    context_trace: &mut ContextTrace,
    canonical: &str,
    action: &str,
) {
    let iteration = trace_steps.last().map(|s| s.iteration).unwrap_or(1);
    trace_steps.push(TraceStep {
        iteration,
        action: action.to_string(),
        output: truncate_with_ellipsis(canonical, 240),
    });
    context_trace.log_event(ContextEvent::Final {
        answer: truncate_with_ellipsis(canonical, 400),
        tokens: ContextTrace::estimate_tokens(canonical),
    });
}
