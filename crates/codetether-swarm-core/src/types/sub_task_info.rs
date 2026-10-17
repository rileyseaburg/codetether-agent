//! SubTaskInfo shared contract.
use super::*;

/// Information about a subtask for display
#[derive(Debug, Clone)]
///
/// # Examples
/// ```
/// let info = codetether_swarm_core::SubTaskInfo { id: "task-1".into(), name: "example".into(), status: codetether_swarm_core::SubTaskStatus::Pending, stage: 0, dependencies: vec![], agent_name: None, current_tool: None, steps: 0, max_steps: 10, tool_call_history: vec![], messages: vec![], output: None, error: None, started_at: None, elapsed_secs: None };
/// assert_eq!(info.steps, 0);
/// ```
pub struct SubTaskInfo {
    pub id: String,
    pub name: String,
    pub status: SubTaskStatus,
    pub stage: usize,
    pub dependencies: Vec<String>,
    pub agent_name: Option<String>,
    pub current_tool: Option<String>,
    pub steps: usize,
    pub max_steps: usize,
    /// Tool call history for this sub-agent
    pub tool_call_history: Vec<AgentToolCallDetail>,
    /// Conversation messages for this sub-agent
    pub messages: Vec<AgentMessageEntry>,
    /// Final output text
    pub output: Option<String>,
    /// Error message if failed
    pub error: Option<String>,
    /// When this agent started running (set on AgentStarted), for live elapsed.
    #[doc(hidden)]
    pub started_at: Option<std::time::Instant>,
    /// Total wall-clock seconds the agent ran (set on completion).
    pub elapsed_secs: Option<u64>,
}
