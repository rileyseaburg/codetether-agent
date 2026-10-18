//! View state and event reducer for the audit-loop view.

use super::audit_loop_model::Iteration;

/// Mutable state backing the audit-loop view.
/// # Examples
/// ```
/// let state = codetether_audit_loop_view::audit_loop_state::AuditLoopState::new();
/// assert!(!state.active);
/// ```
#[derive(Debug, Default)]
pub struct AuditLoopState {
    /// Whether the view is active/has data.
    pub active: bool,
    /// Task label being worked.
    pub task: String,
    /// Retry cap.
    pub max_iterations: usize,
    /// All iterations, oldest first.
    pub iterations: Vec<Iteration>,
    /// Whether the loop has finished.
    pub complete: bool,
    /// Final pass/fail once complete.
    pub passed: bool,
    /// Selected iteration index for navigation.
    pub selected: usize,
}

impl AuditLoopState {
    /// Create an empty state.
    pub fn new() -> Self {
        Self::default()
    }
}
