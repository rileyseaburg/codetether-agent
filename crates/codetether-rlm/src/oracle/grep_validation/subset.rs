//! Apply the coverage threshold to subset claims.

use crate::oracle::{OracleResult, ValidatedTrace};
pub(super) fn classify(
    claimed: usize,
    actual: usize,
    threshold: f32,
    base: impl FnOnce() -> ValidatedTrace,
) -> OracleResult {
    let coverage = claimed as f32 / actual.max(1) as f32;
    if coverage >= threshold {
        super::outcome::golden(base)
    } else {
        let diff = format!(
            "Subset match: model claimed {} but source has {} (coverage: {:.1}%)",
            claimed,
            actual,
            coverage * 100.0
        );
        super::outcome::failed(diff, base)
    }
}
