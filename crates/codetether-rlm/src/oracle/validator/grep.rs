//! Trace validator grep.

use super::TraceValidator;
use super::{OracleResult, ValidatedTrace, VerificationMethod};
impl TraceValidator {
    pub(super) fn oracle_result_from_grep_verification(
        &self,
        verification: crate::oracle::grep_oracle::GrepVerification,
        base_trace: impl FnOnce() -> ValidatedTrace,
    ) -> OracleResult {
        use crate::oracle::grep_oracle::GrepVerification;

        match verification {
            GrepVerification::ExactMatch | GrepVerification::UnorderedMatch => {
                let mut trace = base_trace();
                trace.verification_method = VerificationMethod::GrepOracle;
                trace.verdict = "golden".to_string();
                OracleResult::Golden(trace)
            }
            GrepVerification::CannotVerify { reason } => {
                let mut trace = base_trace();
                trace.verdict = "unverified".to_string();
                OracleResult::Unverified { reason, trace }
            }
            _ => {
                let diff = format!("Grep verification failed: {:?}", verification);
                let mut trace = base_trace();
                trace.verification_method = VerificationMethod::GrepOracle;
                trace.verdict = "failed".to_string();
                trace.oracle_diff = Some(diff.clone());
                OracleResult::Failed {
                    reason: diff.clone(),
                    diff: Some(diff),
                    trace,
                }
            }
        }
    }
}
