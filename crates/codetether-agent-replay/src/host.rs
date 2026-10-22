//! Native session policy, tool execution and recording effects.
use crate::*;
/// Bind replay to a native agent and its live session.
/// # Implementors
/// Keep policy checks and tool-input preparation in the native execution path.
#[async_trait::async_trait]
pub trait Host: Send + Sync {
    /// Native session retaining messages and tool usage.
    type Session: Send + Sync;
    /// Persist pending transcript entries before an external context edit.
    /// # Errors
    /// The edit is not executed if saving fails.
    async fn persist_session(session: &mut Self::Session) -> anyhow::Result<()>;
    /// Refresh the session after an externally committed context edit.
    /// # Errors
    /// Stops replay rather than continuing on stale source state.
    async fn refresh_session(session: &mut Self::Session) -> anyhow::Result<()>;
    /// Return the existing session-policy denial for one serialized tool call.
    fn blocked(session: &Self::Session, name: &str, arguments: &str) -> Option<ToolResult>;
    /// Execute one permitted call through native input preparation and dispatch.
    async fn execute_tool_for_session(
        &self,
        session: &Self::Session,
        id: &str,
        name: &str,
        arguments: &str,
    ) -> ToolResult;
    /// Append a completed tool usage record.
    fn record_tool(session: &mut Self::Session, usage: ToolUse);
    /// Append the corresponding tool-role message and any images.
    fn add_message(session: &mut Self::Session, message: Message);
}
