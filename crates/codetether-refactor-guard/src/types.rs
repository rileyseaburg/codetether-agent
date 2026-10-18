//! Serializable refactor file metrics and violations.
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
/// File metrics and violations passed to an optional continuation hook.
/// # Examples
/// ```
/// let report = codetether_refactor_guard::GuardReport::new(vec![]);
/// assert!(report.violations.is_empty());
/// ```
pub struct GuardReport {
    /// Source files considered by the guard.
    pub files: Vec<GuardFile>,
    /// Violations found by built-in rules and hooks.
    pub violations: Vec<GuardViolation>,
}

#[derive(Debug, Clone, Serialize)]
/// Current and baseline source metrics for one configured file.
/// # Examples
/// ```
/// let file = codetether_refactor_guard::GuardFile { path: "x.rs".into(), status: codetether_refactor_guard::FileStatus::Added, old_code_lines: None, new_code_lines: 1, limit: 50, wrapper_target: None, old_text: None, new_text: "fn main() {}".into() };
/// assert_eq!(file.new_code_lines, 1);
/// ```
pub struct GuardFile {
    /// Workspace-relative source or hook path.
    pub path: String,
    /// Whether Git HEAD contains the file.
    pub status: FileStatus,
    /// Baseline code-line count when present.
    pub old_code_lines: Option<usize>,
    /// Current code-line count.
    pub new_code_lines: usize,
    /// Configured maximum code-line count.
    pub limit: usize,
    /// Detected delegated component or module name.
    pub wrapper_target: Option<String>,
    #[serde(skip_serializing)]
    /// Baseline source, excluded from serialized hook input.
    pub old_text: Option<String>,
    #[serde(skip_serializing)]
    /// Current source, excluded from serialized hook input.
    pub new_text: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
/// Whether a current source file has a baseline in Git HEAD.
/// # Examples
/// ```
/// let status = codetether_refactor_guard::FileStatus::Added;
/// assert!(matches!(status, codetether_refactor_guard::FileStatus::Added));
/// ```
pub enum FileStatus {
    /// No baseline source was found.
    Added,
    /// Existing tracked source changed.
    Modified,
}

#[derive(Debug, Clone, Serialize)]
/// One actionable violation associated with a path.
/// # Examples
/// ```
/// let violation = codetether_refactor_guard::GuardViolation::new("x.rs", "too large");
/// assert_eq!(violation.path, "x.rs");
/// ```
pub struct GuardViolation {
    /// Workspace-relative source or hook path.
    pub path: String,
    /// Explanation that should guide continued work.
    pub message: String,
}
