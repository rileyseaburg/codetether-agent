use crate::SubTaskResult;
use serde::{Deserialize, Serialize};

/// Message from sub-agent to orchestrator
#[derive(Debug, Clone, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// let message = codetether_swarm_core::SubAgentMessage::ToolCall { subagent_id: "agent".into(), tool_name: "read".into(), success: true };
/// assert!(matches!(message, codetether_swarm_core::SubAgentMessage::ToolCall { success: true, .. }));
/// ```
pub enum SubAgentMessage {
    /// Progress update
    Progress {
        subagent_id: String,
        subtask_id: String,
        steps: usize,
        status: String,
    },

    /// Tool call made
    ToolCall {
        subagent_id: String,
        tool_name: String,
        success: bool,
    },

    /// Subtask completed
    Completed {
        subagent_id: String,
        result: SubTaskResult,
    },

    /// Request for resources
    ResourceRequest {
        subagent_id: String,
        resource_type: String,
        resource_id: String,
    },
}
