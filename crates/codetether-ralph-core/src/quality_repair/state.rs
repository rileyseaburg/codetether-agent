//! Evolving output and verification evidence across repair attempts.
/// Output and verification status of one parallel story.
///
/// # Examples
/// ```
/// let state = codetether_ralph_core::quality_repair::QualityProgress {
///     output: "initial".into(), qg_passed: false, failure_reason: None,
/// };
/// assert!(!state.qg_passed);
/// ```
pub struct QualityProgress {
    pub output: String,
    pub qg_passed: bool,
    pub failure_reason: Option<String>,
}
