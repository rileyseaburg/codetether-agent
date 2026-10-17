//! RalphStoryInfo contract.
use super::*;

// ---------------------------------------------------------------------------
// Story info
// ---------------------------------------------------------------------------

/// Information about a story for display
#[derive(Debug, Clone)]
///
/// # Examples
/// ```
/// let story = codetether_ralph_core::events::RalphStoryInfo { id: "US-1".into(), title: "Example".into(), status: codetether_ralph_core::events::RalphStoryStatus::Pending, priority: 1, depends_on: vec![], quality_checks: vec![], tool_call_history: vec![], messages: vec![], output: None, error: None, merge_summary: None, steps: 0, current_tool: None };
/// assert_eq!(story.steps, 0);
/// ```
pub struct RalphStoryInfo {
    pub id: String,
    pub title: String,
    pub status: RalphStoryStatus,
    pub priority: u8,
    pub depends_on: Vec<String>,
    /// Quality check results: (name, passed)
    pub quality_checks: Vec<(String, bool)>,
    /// Tool call history from sub-agent
    pub tool_call_history: Vec<AgentToolCallDetail>,
    /// Conversation messages from sub-agent
    pub messages: Vec<AgentMessageEntry>,
    /// Final output text
    pub output: Option<String>,
    /// Error message if failed
    pub error: Option<String>,
    /// Merge result summary
    pub merge_summary: Option<String>,
    /// Step counter for agent
    pub steps: usize,
    /// Current tool being executed
    pub current_tool: Option<String>,
}
