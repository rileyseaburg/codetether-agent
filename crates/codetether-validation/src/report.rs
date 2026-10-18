//! Structured continuation instructions for unresolved file diagnostics.

/// Diagnostic count and continuation instructions for the current edits.
/// # Examples
/// ```
/// let report = codetether_validation::ValidationReport { issue_count: 1, prompt: "Fix diagnostic".into() };
/// assert_eq!(report.issue_count, 1);
/// ```
#[derive(Debug, Clone)]
pub struct ValidationReport {
    /// Total unique diagnostic count, before display truncation.
    pub issue_count: usize,
    /// Instructions listing affected files and bounded diagnostics.
    pub prompt: String,
}
