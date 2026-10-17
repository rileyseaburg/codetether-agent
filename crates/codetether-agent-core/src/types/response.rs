//! Final execution response and provider usage.
use super::ToolUse;
use serde::{Deserialize, Serialize};

/// Final response returned from a completed agent run.
///
/// This includes the textual answer along with recorded tool activity and
/// provider usage metadata captured in the session.
///
/// # Examples
///
/// ```rust
/// use codetether_agent_core::AgentResponse;
/// let response = AgentResponse { text: String::new(), tool_uses: vec![], usage: Default::default() };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResponse {
    pub text: String,
    pub tool_uses: Vec<ToolUse>,
    pub usage: codetether_provider_core::Usage,
}
