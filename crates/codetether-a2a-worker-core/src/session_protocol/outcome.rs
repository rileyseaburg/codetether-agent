//! Outcome data for a worker session's tool loop.

/// Outcome of a worker session's tool loop.
///
/// # Examples
/// ```rust
/// use codetether_a2a_worker_core::session_protocol::SessionStepsOutcome;
/// let outcome = SessionStepsOutcome { text: "done".into(), budget_exhausted: false, max_steps: 50 };
/// assert!(!outcome.budget_exhausted);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionStepsOutcome {
    /// Final assistant text (trimmed).
    pub text: String,
    /// `true` when the loop stopped because the step budget ran out before
    /// the agent signalled completion. Callers must not report such a run as
    /// completed work.
    pub budget_exhausted: bool,
    /// Step budget that applied to this run.
    pub max_steps: usize,
}
