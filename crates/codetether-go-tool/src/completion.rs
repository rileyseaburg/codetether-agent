//! Final fields needed to report a completed autonomous pipeline.

/// Completed story counts, branch and rendered pipeline summary.
///
/// # Examples
/// ```
/// let result = codetether_go_tool::Completion {
///     passed: 1, total: 1, all_passed: true,
///     feature_branch: "feature".into(), summary: "Completed".into(),
/// };
/// assert!(result.all_passed);
/// ```
pub struct Completion {
    /// Count of successful stories.
    pub passed: usize,
    /// Total story count.
    pub total: usize,
    /// Whether all stories succeeded.
    pub all_passed: bool,
    /// Branch produced by the pipeline.
    pub feature_branch: String,
    /// Rendered pipeline result.
    pub summary: String,
}
