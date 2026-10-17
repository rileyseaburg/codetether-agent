//! Owned output and verification status across quality-repair attempts.
pub(super) struct QualityProgress {
    pub(super) output: String,
    pub(super) qg_passed: bool,
    pub(super) failure_reason: Option<String>,
}
