//! User-facing continuation report produced by guard evaluation.
/// Refactor violations ready for the calling validation pipeline.
/// # Examples
/// ```
/// let report = codetether_refactor_guard::Report { issue_count: 1, prompt: "Split the oversized file".into() };
/// assert_eq!(report.issue_count, 1);
/// ```
pub struct Report {
    /// Number of individual guard violations.
    pub issue_count: usize,
    /// Formatted continuation instructions.
    pub prompt: String,
}
