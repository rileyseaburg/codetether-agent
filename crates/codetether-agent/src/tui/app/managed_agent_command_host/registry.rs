//! Bind parent-scoped registry operations and chat dispatch.
use super::Native;
use crate::tui::app::managed_agent;
use codetether_agent_command::Registry;
impl Registry for Native<'_> {
    fn owned(&self, name: &str) -> bool {
        self.parent
            .is_some_and(|parent| managed_agent::owned(parent, name))
    }
    async fn remove(&mut self, name: &str) -> codetether_tool_core::ToolResult {
        managed_agent::kill(self.parent.expect("agent removal parent"), name).await
    }
    async fn send(&mut self, name: &str, message: &str) {
        managed_agent::chat::send(
            self.app,
            self.parent.expect("agent message parent"),
            name,
            message,
        )
        .await;
    }
}
