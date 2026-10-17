//! Load a claimed grep payload and evaluate it against source.

use super::grep_oracle::GrepOracle;
use super::schema::FinalPayload;
use super::trace_types::{OracleResult, ValidatedTrace};
mod compare;
mod outcome;
mod subset;
mod verdict;

/// Validate structured grep claims against source matches.
///
/// # Examples
/// ```
/// use codetether_rlm::{TraceValidator, RlmAnalysisResult, OracleResult};
/// let result = RlmAnalysisResult { answer: "not JSON".into(), iterations: 1,
///     sub_queries: vec![], stats: Default::default() };
/// let verdict = TraceValidator::new().validate(&result, "", None, None, None);
/// assert!(matches!(verdict, OracleResult::Unverified { .. }));
/// ```
pub fn validate_grep_payload(
    payload: &FinalPayload,
    source: &str,
    confidence_threshold: f32,
    base_trace: impl FnOnce() -> ValidatedTrace,
) -> OracleResult {
    let grep_payload = match payload {
        FinalPayload::Grep(g) => g,
        _ => {
            let mut t = base_trace();
            t.verdict = "failed".into();
            return OracleResult::Failed {
                reason: "Expected grep payload".into(),
                diff: None,
                trace: t,
            };
        }
    };

    let oracle = GrepOracle::new(source.to_string());
    let ground_truth = match oracle.grep(&grep_payload.pattern) {
        Ok(matches) => matches,
        Err(error) => {
            let mut trace = base_trace();
            trace.verdict = "unverified".into();
            return OracleResult::Unverified {
                reason: format!("Could not run grep: {error}"),
                trace,
            };
        }
    };
    compare::compare(
        &oracle,
        grep_payload,
        &ground_truth,
        confidence_threshold,
        base_trace,
    )
}
