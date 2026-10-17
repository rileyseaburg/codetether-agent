use super::*;
/// Check step exhaustion before checking the deadline.
///
/// # Arguments
/// * `steps` - Completed turn count.
/// * `max_steps` - Allowed turn count.
/// * `deadline` - Absolute deadline.
/// * `timeout_secs` - Configured timeout used in diagnostics.
///
/// # Returns
/// The first reached limit, or None.
///
/// # Examples
/// See the module overview for how this operation participates in an agent turn.
pub fn limit_reached(
    steps: usize,
    max_steps: usize,
    deadline: Instant,
    timeout_secs: u64,
) -> Option<AgentLoopExit> {
    if steps >= max_steps {
        tracing::warn!(max_steps = max_steps, "Sub-agent reached step limit");
        Some(AgentLoopExit::MaxStepsReached)
    } else if Instant::now() > deadline {
        tracing::warn!(timeout_secs = timeout_secs, "Sub-agent timed out");
        Some(AgentLoopExit::TimedOut)
    } else {
        None
    }
}
