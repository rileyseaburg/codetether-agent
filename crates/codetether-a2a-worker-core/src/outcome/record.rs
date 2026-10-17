//! Final claimed-task result fields.
/// Normalized outcome ready for task completion reporting.
///
/// # Examples
/// ```rust
/// use codetether_a2a_worker_core::outcome::TaskOutcome;
/// let outcome = TaskOutcome { status: "completed", result: None, error: None, session_id: None };
/// assert_eq!(outcome.status, "completed");
/// ```

pub struct TaskOutcome {
    /// Completion status reported to the control plane.
    pub status: &'static str,
    /// Optional human-readable task result.
    pub result: Option<String>,
    /// Optional task error detail.
    pub error: Option<String>,
    /// Optional session backing the execution.
    pub session_id: Option<String>,
}
