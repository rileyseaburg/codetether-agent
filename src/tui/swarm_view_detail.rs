//! Per-sub-agent detail records shown in the swarm view.

/// A recorded tool call for a sub-agent.
#[derive(Debug, Clone)]
pub struct AgentToolCallDetail {
    pub tool_name: String,
    pub input_preview: String,
    pub output_preview: String,
    pub success: bool,
}

/// A conversation message entry for a sub-agent.
#[derive(Debug, Clone)]
pub struct AgentMessageEntry {
    pub role: String,
    pub content: String,
    pub is_tool_call: bool,
}
