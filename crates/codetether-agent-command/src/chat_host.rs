//! Native state and message operations for direct child chat.
use crate::State;
use codetether_chat_model::ChatMessage;
use codetether_tool_core::ToolResult;
use std::future::Future;
/// Runtime callbacks for parent-scoped direct chat.
/// # Implementors
/// Use the active session ID and existing child-tool request dispatch.
pub trait ChatHost: State {
    /// Clone the current parent session ID if one is installed.
    fn parent(&self) -> Option<String>;
    /// Check child ownership for an explicit parent.
    fn owned_by(&self, parent: &str, name: &str) -> bool;
    /// Dispatch a message to the named child under this parent.
    fn message(
        &mut self,
        parent: &str,
        name: &str,
        message: &str,
    ) -> impl Future<Output = ToolResult>;
    /// Append a chat message to the visible transcript.
    fn push(&mut self, message: ChatMessage);
    /// Clear the input after message dispatch.
    fn clear_input(&mut self);
}
