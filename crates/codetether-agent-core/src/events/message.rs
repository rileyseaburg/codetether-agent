//! AgentMessageEntry event payload.

/// A conversation message entry for a sub-agent.
#[derive(Debug, Clone)]
///
/// # Examples
/// ```
/// let entry = codetether_agent_core::events::AgentMessageEntry { role: "assistant".into(), content: "done".into(), is_tool_call: false };
/// assert!(!entry.is_tool_call);
/// ```
pub struct AgentMessageEntry {
    /// Message role label.
    pub role: String,
    /// Message text.
    pub content: String,
    /// Whether this entry represents a tool call.
    pub is_tool_call: bool,
}
