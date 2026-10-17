//! Legacy executor extract latest grep matches.
use super::*;

/// Select the latest grep evidence, then fall back to numbered output.
///
/// # Arguments
/// * `trace_steps` - Recorded steps in execution order.
/// # Returns
/// Parsed matches, including an empty vector for an explicit no-match result.
/// # Examples
/// ```
/// assert!(codetether_rlm::legacy_support::extract_latest_grep_matches(&[]).is_none());
/// ```
pub fn extract_latest_grep_matches(trace_steps: &[TraceStep]) -> Option<Vec<(usize, String)>> {
    for step in trace_steps.iter().rev() {
        if !step.action.contains("grep(") && !step.action.contains("rlm_grep(") {
            continue;
        }
        if step.output.trim() == "(no matches)" {
            return Some(Vec::new());
        }
        let parsed = parse_line_numbered_output(&step.output);
        if !parsed.is_empty() {
            return Some(parsed);
        }
    }

    for step in trace_steps.iter().rev() {
        let parsed = parse_line_numbered_output(&step.output);
        if !parsed.is_empty() {
            return Some(parsed);
        }
    }

    None
}
