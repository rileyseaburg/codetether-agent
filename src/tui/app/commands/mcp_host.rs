//! Native connection registry and message bindings for MCP commands.
use crate::tui::app::state::App;
pub(super) struct Native<'a>(pub &'a mut App);
impl codetether_mcp_command::interactive::Host for Native<'_> {
    async fn execute(&mut self, raw: &str) -> codetether_mcp_command::CommandOutput {
        codetether_mcp_command::execute(&self.0.state.mcp_registry, raw).await
    }
    fn set_status(&mut self, status: String) {
        self.0.state.status = status;
    }
    fn push_system_message(&mut self, message: String) {
        super::push_system_message(self.0, message);
    }
}
