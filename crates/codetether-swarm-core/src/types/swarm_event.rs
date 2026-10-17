//! SwarmEvent shared contract.
use super::*;

/// Events emitted by swarm execution for TUI updates
#[derive(Debug, Clone)]
///
/// # Examples
/// ```
/// let event = codetether_swarm_core::SwarmEvent::Error("failed".into());
/// match event { codetether_swarm_core::SwarmEvent::Error(text) => assert_eq!(text, "failed"), _ => unreachable!() }
/// ```
pub enum SwarmEvent {
    /// Swarm execution started
    Started { task: String, total_subtasks: usize },
    /// Task decomposition complete
    Decomposed { subtasks: Vec<SubTaskInfo> },
    /// SubTask status changed
    SubTaskUpdate {
        id: String,
        name: String,
        status: SubTaskStatus,
        agent_name: Option<String>,
    },
    /// SubAgent started working
    AgentStarted {
        subtask_id: String,
        agent_name: String,
        specialty: String,
    },
    /// SubAgent made a tool call (basic)
    AgentToolCall {
        subtask_id: String,
        tool_name: String,
    },
    /// SubAgent made a tool call with detail
    AgentToolCallDetail {
        subtask_id: String,
        detail: AgentToolCallDetail,
    },
    /// SubAgent sent/received a message
    AgentMessage {
        subtask_id: String,
        entry: AgentMessageEntry,
    },
    /// SubAgent completed
    AgentComplete {
        subtask_id: String,
        success: bool,
        steps: usize,
    },
    /// SubAgent produced output text
    AgentOutput { subtask_id: String, output: String },
    /// SubAgent encountered an error
    AgentError { subtask_id: String, error: String },
    /// Stage completed
    StageComplete {
        stage: usize,
        completed: usize,
        failed: usize,
    },
    /// Swarm execution complete
    Complete { success: bool, stats: SwarmStats },
    /// Error occurred
    Error(String),
}
