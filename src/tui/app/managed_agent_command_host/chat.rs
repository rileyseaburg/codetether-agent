//! Parent-scoped chat callbacks for the shared interaction flow.
use super::Native;
use crate::tui::app::managed_agent;
use codetether_agent_command::ChatHost;
impl ChatHost for Native<'_> {
    fn parent(&self) -> Option<String> {
        self.app.state.session_id.clone()
    }
    fn owned_by(&self, parent: &str, name: &str) -> bool {
        managed_agent::owned(parent, name)
    }
    async fn message(
        &mut self,
        parent: &str,
        name: &str,
        message: &str,
    ) -> codetether_tool_core::ToolResult {
        managed_agent::message(parent, name, message).await
    }
    fn push(&mut self, message: codetether_chat_model::ChatMessage) {
        self.app.state.messages.push(message);
    }
    fn clear_input(&mut self) {
        self.app.state.composer.clear_input();
    }
}
