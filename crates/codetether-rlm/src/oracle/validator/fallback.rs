//! Unverifiable semantic answers and malformed payload routing.

use super::{OracleResult, TraceValidator, ValidatedTrace};
pub(super) fn semantic(base_trace: impl FnOnce() -> ValidatedTrace) -> OracleResult {
    let mut trace = base_trace();
    trace.verdict = "unverified".to_string();
    OracleResult::Unverified {
        reason: "Semantic queries require LLM understanding - no deterministic oracle available"
            .to_string(),
        trace,
    }
}
impl TraceValidator {
    pub(super) fn malformed(
        &self,
        error: &str,
        raw: &str,
        query: &str,
        source: &str,
        answer: &str,
        base_trace: impl FnOnce() -> ValidatedTrace,
    ) -> OracleResult {
        let trimmed = raw.trim_start();
        if trimmed.starts_with('{') || trimmed.starts_with('[') {
            let mut trace = base_trace();
            trace.verdict = "failed".to_string();
            OracleResult::Failed {
                reason: format!("Malformed FINAL payload: {}", error),
                diff: None,
                trace,
            }
        } else {
            self.validate_plain_text(&query, source, answer, base_trace)
        }
    }
}
