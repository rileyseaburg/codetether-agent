//! Trace validator plain.

use super::TraceValidator;
use super::{GrepOracle, OracleResult, ValidatedTrace};
impl TraceValidator {
    pub(super) fn validate_plain_text(
        &self,
        query: &str,
        source: &str,
        answer: &str,
        base_trace: impl FnOnce() -> ValidatedTrace,
    ) -> OracleResult {
        match GrepOracle::classify_query(query) {
            crate::oracle::QueryType::PatternMatch => {
                let oracle = GrepOracle::new(source.to_string());
                let verification = oracle.verify(answer, query);
                self.oracle_result_from_grep_verification(verification, base_trace)
            }
            crate::oracle::QueryType::Structural => {
                let mut trace = base_trace();
                trace.verdict = "unverified".to_string();
                OracleResult::Unverified {
                    reason: "Structured query result was not emitted as FINAL(JSON)".to_string(),
                    trace,
                }
            }
            crate::oracle::QueryType::Semantic => {
                let mut trace = base_trace();
                trace.verdict = "unverified".to_string();
                OracleResult::Unverified {
                    reason: "Semantic query - no deterministic oracle available".to_string(),
                    trace,
                }
            }
        }
    }
}
