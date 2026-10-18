//! Direct UI state access for agent commands.
use super::Native;
use codetether_agent_command::State;
impl State for Native<'_> {
    fn status(&mut self, text: String) {
        self.app.state.status = text;
    }
    fn active(&self) -> Option<&str> {
        self.app.state.active_spawned_agent.as_deref()
    }
    fn set_active(&mut self, name: Option<String>) {
        self.app.state.active_spawned_agent = name;
    }
    fn legacy_exists(&self, name: &str) -> bool {
        self.app.state.spawned_agents.contains_key(name)
    }
    fn remove_legacy(&mut self, name: &str) -> bool {
        self.app.state.spawned_agents.remove(name).is_some()
    }
    fn clear_legacy_output(&mut self, name: &str) {
        self.app.state.streaming_agent_texts.remove(name);
    }
    fn swarm_names(&self) -> Vec<Option<String>> {
        self.app
            .state
            .swarm
            .subtasks
            .iter()
            .map(|task| task.agent_name.clone())
            .collect()
    }
    fn select_swarm(&mut self, index: usize) {
        self.select_worker(index);
    }
    fn navigate_agents(&mut self) {
        crate::tui::app::navigation::handle_tab(self.app);
    }
    fn present(
        &mut self,
        result: &codetether_tool_core::ToolResult,
        success: String,
        failure: &str,
    ) {
        crate::tui::app::managed_agent::ui::present(self.app, result, success, failure);
    }
}
