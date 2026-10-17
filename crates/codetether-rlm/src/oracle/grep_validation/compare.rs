//! Convert schema matches into grep claims and classify the comparison.
use crate::oracle::{GrepOracle, GrepPayload, OracleResult, ValidatedTrace};
pub(super) fn compare(
    oracle: &GrepOracle,
    payload: &GrepPayload,
    ground_truth: &[(usize, String)],
    threshold: f32,
    base: impl FnOnce() -> ValidatedTrace,
) -> OracleResult {
    let claimed: Vec<_> = payload
        .matches
        .iter()
        .map(|m| (m.line, m.text.clone()))
        .collect();
    super::verdict::classify(
        oracle.verify_matches(&claimed, ground_truth),
        threshold,
        base,
    )
}
