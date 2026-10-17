//! Construct grep-verification trace outcomes.

use crate::oracle::{OracleResult, ValidatedTrace, VerificationMethod};
pub(super) fn golden(base: impl FnOnce() -> ValidatedTrace) -> OracleResult {
    let mut trace = base();
    trace.verification_method = VerificationMethod::GrepOracle;
    trace.verdict = "golden".into();
    OracleResult::Golden(trace)
}
pub(super) fn failed(diff: String, base: impl FnOnce() -> ValidatedTrace) -> OracleResult {
    let mut trace = base();
    trace.verification_method = VerificationMethod::GrepOracle;
    trace.verdict = "failed".into();
    trace.oracle_diff = Some(diff.clone());
    OracleResult::Failed {
        reason: diff.clone(),
        diff: Some(diff),
        trace,
    }
}
pub(super) fn unverified(reason: String, base: impl FnOnce() -> ValidatedTrace) -> OracleResult {
    let mut trace = base();
    trace.verdict = "unverified".into();
    OracleResult::Unverified { reason, trace }
}
