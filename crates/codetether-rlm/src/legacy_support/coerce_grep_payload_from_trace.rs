//! Legacy executor coerce grep payload from trace.
use super::*;

/// Create a canonical grep payload from recorded evidence.
///
/// # Arguments
/// * `query` - Original analysis question.
/// * `trace_steps` - Ordered execution evidence.
/// # Returns
/// Serialized grep payload when pattern and evidence are available.
/// # Examples
/// ```
/// assert!(codetether_rlm::legacy_support::coerce_grep_payload_from_trace("find pattern", &[]).is_none());
/// ```
pub fn coerce_grep_payload_from_trace(query: &str, trace_steps: &[TraceStep]) -> Option<String> {
    if GrepOracle::classify_query(query) != QueryType::PatternMatch {
        return None;
    }

    let pattern = GrepOracle::infer_pattern(query)?;
    let matches = extract_latest_grep_matches(trace_steps)?;
    let file = infer_file_from_query(query).unwrap_or_else(|| "unknown".to_string());

    let payload = FinalPayload::Grep(GrepPayload {
        file,
        pattern,
        matches: matches
            .into_iter()
            .map(|(line, text)| GrepMatch { line, text })
            .collect(),
    });

    serde_json::to_string(&payload).ok()
}
