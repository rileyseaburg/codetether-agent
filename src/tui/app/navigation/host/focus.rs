//! Native agent lookup and UI field bindings for shared focus policy.
use super::Navigation;
use crate::tui::models::ViewMode;
impl codetether_tui_navigation::focus::Host for Navigation<'_> {
    fn apply_selected_slash_suggestion(&mut self) -> bool {
        self.0.state.composer.apply_selected_slash_suggestion()
    }
    fn agent_names(&self) -> Vec<String> {
        super::super::agent_focus::agent_names(self.0)
    }
    fn active_agent(&self) -> Option<String> {
        self.0.state.agent_tree.active.clone()
    }
    fn swarm_names(&self) -> Vec<Option<String>> {
        self.0
            .state
            .swarm
            .subtasks
            .iter()
            .map(|task| task.agent_name.clone())
            .collect()
    }
    fn selected_swarm_agent(&self) -> Option<String> {
        self.0
            .state
            .swarm
            .selected_subtask()
            .and_then(|task| task.agent_name.clone())
    }
    fn set_active_agent(&mut self, next: Option<String>) {
        self.0.state.agent_tree.active = next;
    }
    fn select_swarm_agent(&mut self, index: usize) {
        self.0.state.swarm.selected_index = index;
        self.0.state.swarm.list_state.select(Some(index));
    }
    fn show_swarm(&mut self) {
        self.0.state.set_view_mode(ViewMode::Swarm);
    }
    fn enter_swarm_detail(&mut self) {
        self.0.state.swarm.enter_detail();
    }
    fn clear_message_cache(&mut self) {
        crate::tui::app::message_cache_invalidate::clear(&mut self.0.state);
    }
    fn scroll_to_bottom(&mut self) {
        self.0.state.scroll_to_bottom();
    }
}
