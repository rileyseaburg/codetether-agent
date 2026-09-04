//! Outcome data for a worker session's tool loop.

/// Outcome of a worker session's tool loop.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::a2a::worker) struct SessionStepsOutcome {
    /// Final assistant text (trimmed).
    pub text: String,
    /// `true` when the loop stopped because the step budget ran out before
    /// the agent signalled completion. Callers must not report such a run as
    /// completed work.
    pub budget_exhausted: bool,
    /// Step budget that applied to this run.
    pub max_steps: usize,
}
