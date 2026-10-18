//! Serializable outcome for one `swarm_execute` worker.

use super::model_selection::ModelSelection;
use codetether_swarm_core::agent_retry::AgentLoopExit;

#[cfg(test)]
/// Whether a terminated worker produced an acceptable deliverable.
pub fn completed(exit: AgentLoopExit, output: &str) -> bool {
    failure(exit, output).is_none()
}

/// Classify worker termination and delivered output.
/// # Arguments
/// * `exit` — Agent-loop termination reason.
/// * `output` — Worker response text.
/// # Returns
/// A failure message, or `None` for an acceptable completed deliverable.
/// # Examples
/// ```
/// use codetether_swarm_core::agent_retry::AgentLoopExit;
/// assert!(codetether_swarm_tool::task_result::failure(AgentLoopExit::TimedOut, "").is_some());
/// ```
pub fn failure(exit: AgentLoopExit, output: &str) -> Option<String> {
    match exit {
        AgentLoopExit::Completed => codetether_swarm_core::deliverable::deliverable_error(output),
        AgentLoopExit::MaxStepsReached => Some("Sub-agent reached its step limit".into()),
        AgentLoopExit::TimedOut => Some("Sub-agent timed out".into()),
    }
}

/// Serializable outcome and model diagnostics for one worker.
/// # Examples
/// ```
/// let model = codetether_swarm_tool::model_selection::resolve(None, &["openai"]).unwrap();
/// let result = codetether_swarm_tool::task_result::TaskResult::failed("1".into(), "Review".into(), "timeout".into(), model);
/// assert!(!result.success);
/// ```
#[derive(serde::Serialize)]
pub struct TaskResult {
    /// Stable worker identifier.
    pub task_id: String,
    /// Worker display name.
    pub task_name: String,
    /// Whether execution delivered a valid result.
    pub success: bool,
    /// Worker response text.
    pub output: String,
    /// Failure description when unsuccessful.
    pub error: Option<String>,
    /// Provider-turn count.
    pub steps: usize,
    /// Tool invocation count.
    pub tool_calls: usize,
    /// Requested and resolved model diagnostics.
    pub model: ModelSelection,
}

impl TaskResult {
    /// Construct a failed worker record with empty output and zero counters.
    pub fn failed(
        task_id: String,
        task_name: String,
        error: String,
        model: ModelSelection,
    ) -> Self {
        Self {
            task_id,
            task_name,
            success: false,
            output: String::new(),
            error: Some(error),
            steps: 0,
            tool_calls: 0,
            model,
        }
    }
}
