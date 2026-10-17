/// A single validation issue discovered before swarm execution.
#[derive(Debug, Clone)]
///
/// # Examples
/// ```
/// use codetether_swarm_core::validation::{ValidationIssue, IssueSeverity, IssueCategory};
/// let issue = ValidationIssue { severity: IssueSeverity::Info, category: IssueCategory::Configuration, message: "ready".into(), suggestion: None };
/// assert_eq!(issue.message, "ready");
/// ```
pub struct ValidationIssue {
    /// Severity level for this issue.
    pub severity: IssueSeverity,
    /// Validation area that produced this issue.
    pub category: IssueCategory,
    /// Human-readable diagnostic message.
    pub message: String,
    /// Optional remediation guidance.
    pub suggestion: Option<String>,
}

/// Severity levels for validation issues.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
///
/// # Examples
/// ```
/// use codetether_swarm_core::validation::IssueSeverity;
/// let severity = IssueSeverity::Info;
/// match severity { IssueSeverity::Error | IssueSeverity::Warning => {}, IssueSeverity::Info => {} }
/// ```
pub enum IssueSeverity {
    /// Critical issue that prevents execution.
    Error,
    /// Non-fatal issue that may affect execution quality.
    Warning,
    /// Informational diagnostic only.
    Info,
}

/// Categories of validation issues.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
///
/// # Examples
/// ```
/// use codetether_swarm_core::validation::IssueCategory;
/// let category = IssueCategory::Configuration;
/// match category { IssueCategory::Provider | IssueCategory::Workspace | IssueCategory::Configuration | IssueCategory::Dependencies | IssueCategory::TokenEstimate => {} }
/// ```
pub enum IssueCategory {
    /// Provider or credential issue.
    Provider,
    /// Workspace or git state issue.
    Workspace,
    /// Swarm configuration issue.
    Configuration,
    /// Subtask dependency issue.
    Dependencies,
    /// Token usage estimate issue.
    TokenEstimate,
}
