//! Terminal reasons produced by a sub-agent execution loop.

/// Describes why an agent loop stopped.
///
/// Variants:
///
/// * [`Completed`](Self::Completed) — The task finished normally.
/// * [`MaxStepsReached`](Self::MaxStepsReached) — The step budget was exhausted.
/// * [`TimedOut`](Self::TimedOut) — The execution deadline elapsed.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::swarm::executor::AgentLoopExit;
/// let exit = AgentLoopExit::Completed;
/// assert!(matches!(exit, AgentLoopExit::Completed));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentLoopExit {
    /// The provider completed the requested task.
    Completed,
    /// The loop consumed its configured maximum number of steps.
    MaxStepsReached,
    /// The overall agent deadline elapsed.
    TimedOut,
}
