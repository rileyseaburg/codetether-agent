//! Execution evidence shared by forage runtimes.
/// Result detail, changed-file evidence and quality-gate status.
///
/// # Examples
/// ```
/// let outcome = codetether_forage_core::execution::ExecutionOutcome {
///     detail: "complete".into(), changed_files: vec![], quality_gates_passed: false,
/// };
/// assert!(!outcome.quality_gates_passed);
/// ```

#[derive(Debug, Clone)]
pub struct ExecutionOutcome {
    pub detail: String,
    pub changed_files: Vec<String>,
    pub quality_gates_passed: bool,
}
