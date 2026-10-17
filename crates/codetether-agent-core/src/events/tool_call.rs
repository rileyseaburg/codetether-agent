//! AgentToolCallDetail event payload.
//! Per-sub-agent detail records shown in the swarm view.

/// A recorded tool call for a sub-agent.
#[derive(Debug, Clone)]
///
/// # Examples
/// ```
/// let entry = codetether_agent_core::events::AgentToolCallDetail { tool_name: "read".into(), input_preview: "file".into(), output_preview: "text".into(), success: true };
/// assert!(entry.success);
/// ```
pub struct AgentToolCallDetail {
    /// Tool identifier.
    pub tool_name: String,
    /// Input preview for display.
    pub input_preview: String,
    /// Output preview for display.
    pub output_preview: String,
    /// Whether execution succeeded.
    pub success: bool,
}
