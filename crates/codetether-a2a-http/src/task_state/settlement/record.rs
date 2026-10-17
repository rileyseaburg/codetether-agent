//! Inputs for settling one completed inbound task.
use super::*;
/// Task and observation context shared by success and failure settlement.
///
/// # Examples
///
/// ```rust
/// use codetether_a2a_http::task_state::settlement::Settle;
/// let tasks = dashmap::DashMap::new();
/// let settle = Settle {
///     tasks: &tasks, task_id: "task-1", context_id: None,
///     prompt: "hello", blocking: true, elapsed: std::time::Duration::ZERO,
/// };
/// assert_eq!(settle.task_id, "task-1");
/// ```

pub struct Settle<'a> {
    /// Task table receiving the terminal state.
    pub tasks: &'a DashMap<String, Task>,
    /// Identifier of the task to settle.
    pub task_id: &'a str,
    /// Optional conversation context carried into the response.
    pub context_id: Option<&'a str>,
    /// Original prompt recorded with the observation.
    pub prompt: &'a str,
    /// Whether the caller awaited this response.
    pub blocking: bool,
    /// Elapsed time for the completed turn.
    pub elapsed: Duration,
}
