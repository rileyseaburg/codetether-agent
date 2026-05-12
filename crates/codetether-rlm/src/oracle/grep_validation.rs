//! Grep-based payload validation against source text.
//!
//! Compares claimed grep matches from a [`FinalPayload::Grep`]
//! payload with actual source content.  When the match ratio
//! exceeds the confidence threshold the result is promoted to
//! [`OracleResult::Golden`]; otherwise it falls back to
//! [`OracleResult::Unverified`].
//!
//! # Examples
//!
//! ```ignore
//! let result = validate_grep_payload(&payload, src, 0.8, || base);
//! assert!(matches!(result, OracleResult::Golden(_)));
//! ```

use super::grep_oracle::GrepOracle;
use super::schema::FinalPayload;
use super::trace_types::{OracleResult, ValidatedTrace};
use super::types::VerificationMethod;

/// Validate claimed grep matches against raw source text.
///
/// Extracts the grep payload, counts how many claimed match
/// texts actually appear in `source`, and compares the ratio
/// to `confidence_threshold`.  Returns [`OracleResult::Golden`]
/// when the ratio is at or above threshold, otherwise
/// [`OracleResult::Unverified`].  Non-grep payloads produce
/// [`OracleResult::Failed`].
///
/// # Examples
///
/// ```ignore
/// let result = validate_grep_payload(
///     &payload, &source, 0.8, || trace.clone(),
/// );
/// match result {
///     OracleResult::Golden(_) => println!("verified"),
///     _ => println!("not verified"),
/// }
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
    let claimed: Vec<(usize, String)> = grep_payload
        .matches
        .iter()
        .map(|m| (m.line, m.text.clone()))
        .collect();

    match oracle.verify_matches(&claimed, &ground_truth) {
        super::grep_oracle::GrepVerification::ExactMatch
        | super::grep_oracle::GrepVerification::UnorderedMatch => {
            let mut trace = base_trace();
            trace.verification_method = VerificationMethod::GrepOracle;
            trace.verdict = "golden".into();
            OracleResult::Golden(trace)
        }
        super::grep_oracle::GrepVerification::SubsetMatch { claimed, actual } => {
            let coverage = claimed as f32 / actual.max(1) as f32;
            if coverage >= confidence_threshold {
                let mut trace = base_trace();
                trace.verification_method = VerificationMethod::GrepOracle;
                trace.verdict = "golden".into();
                OracleResult::Golden(trace)
            } else {
                let diff = format!(
                    "Subset match: model claimed {} but source has {} (coverage: {:.1}%)",
                    claimed,
                    actual,
                    coverage * 100.0
                );
                let mut trace = base_trace();
                trace.verification_method = VerificationMethod::GrepOracle;
                trace.verdict = "failed".into();
                trace.oracle_diff = Some(diff.clone());
                OracleResult::Failed {
                    reason: diff.clone(),
                    diff: Some(diff),
                    trace,
                }
            }
        }
        super::grep_oracle::GrepVerification::HasFalsePositives { false_positives } => {
            let diff = format!(
                "False positives: {} claims not found in source: {:?}",
                false_positives.len(),
                false_positives
            );
            let mut trace = base_trace();
            trace.verification_method = VerificationMethod::GrepOracle;
            trace.verdict = "failed".into();
            trace.oracle_diff = Some(diff.clone());
            OracleResult::Failed {
                reason: diff.clone(),
                diff: Some(diff),
                trace,
            }
        }
        super::grep_oracle::GrepVerification::HasFalseNegatives { false_negatives } => {
            let diff = format!(
                "False negatives: {} items in source not claimed: {:?}",
                false_negatives.len(),
                false_negatives
            );
            let mut trace = base_trace();
            trace.verification_method = VerificationMethod::GrepOracle;
            trace.verdict = "failed".into();
            trace.oracle_diff = Some(diff.clone());
            OracleResult::Failed {
                reason: diff.clone(),
                diff: Some(diff),
                trace,
            }
        }
        super::grep_oracle::GrepVerification::Mismatch => {
            let diff = "Grep verification mismatch between claimed and source matches".to_string();
            let mut trace = base_trace();
            trace.verification_method = VerificationMethod::GrepOracle;
            trace.verdict = "failed".into();
            trace.oracle_diff = Some(diff.clone());
            OracleResult::Failed {
                reason: diff.clone(),
                diff: Some(diff),
                trace,
            }
        }
        super::grep_oracle::GrepVerification::CannotVerify { reason } => {
            let mut trace = base_trace();
            trace.verdict = "unverified".into();
            OracleResult::Unverified { reason, trace }
        }
    }
}
