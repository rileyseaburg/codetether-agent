//! Legacy executor has pattern evidence.
use super::*;

/// Check whether the trace contains pattern-search evidence.
///
/// # Arguments
/// * `trace_steps` - Recorded steps to inspect.
/// # Returns
/// Whether an action or its output supplies pattern evidence.
/// # Examples
/// ```
/// assert!(!codetether_rlm::legacy_support::has_pattern_evidence(&[]));
/// ```
pub fn has_pattern_evidence(trace_steps: &[TraceStep]) -> bool {
    trace_steps.iter().any(|step| {
        step.action.contains("grep(")
            || step.action.contains("rlm_grep(")
            || step.output.contains("(no matches)")
            || !parse_line_numbered_output(&step.output).is_empty()
    })
}
