//! Map grep comparison variants to trace verdicts.

use crate::oracle::{GrepVerification, OracleResult, ValidatedTrace};
pub(super) fn classify(
    verification: GrepVerification,
    threshold: f32,
    base: impl FnOnce() -> ValidatedTrace,
) -> OracleResult {
    match verification {
        GrepVerification::ExactMatch | GrepVerification::UnorderedMatch => {
            super::outcome::golden(base)
        }
        GrepVerification::SubsetMatch { claimed, actual } => {
            super::subset::classify(claimed, actual, threshold, base)
        }
        GrepVerification::HasFalsePositives { false_positives } => super::outcome::failed(
            format!(
                "False positives: {} claims not found in source: {:?}",
                false_positives.len(),
                false_positives
            ),
            base,
        ),
        GrepVerification::HasFalseNegatives { false_negatives } => super::outcome::failed(
            format!(
                "False negatives: {} items in source not claimed: {:?}",
                false_negatives.len(),
                false_negatives
            ),
            base,
        ),
        GrepVerification::Mismatch => super::outcome::failed(
            "Grep verification mismatch between claimed and source matches".to_string(),
            base,
        ),
        GrepVerification::CannotVerify { reason } => super::outcome::unverified(reason, base),
    }
}
