//! Native operations for a managed child spawn command.
use crate::{
    session::Session,
    tool::ToolResult,
    tui::app::{managed_agent, state::App},
};
use std::path::Path;
pub(super) struct Native<'a> {
    pub app: &'a mut App,
    pub session: &'a Session,
    pub cwd: &'a Path,
}
impl codetether_agent_command::spawn::Host for Native<'_> {
    fn status(&mut self, text: String) {
        self.app.state.status = text;
    }
    async fn spawn(&mut self, name: &str, instructions: &str) -> ToolResult {
        managed_agent::spawn(self.session, self.cwd, name, instructions).await
    }
    fn watch(&mut self, name: &str) {
        super::watch::open(self.app, name);
    }
    fn present(&mut self, result: &ToolResult, success: String, failure: &str) {
        managed_agent::ui::present(self.app, result, success, failure);
    }
}
