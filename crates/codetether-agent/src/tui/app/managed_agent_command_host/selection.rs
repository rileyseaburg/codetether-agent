//! Apply the selected worker to the native detail view.
use super::Native;
use crate::tui::models::ViewMode;
impl Native<'_> {
    pub(super) fn select_worker(&mut self, index: usize) {
        self.app.state.swarm.selected_index = index;
        self.app.state.swarm.list_state.select(Some(index));
        self.app.state.swarm.enter_detail();
        crate::tui::app::state::view::transition(&mut self.app.state)
            .set_view_mode(ViewMode::Swarm);
        self.app.state.agent_tree.active = self.app.state.swarm.subtasks[index].agent_name.clone();
    }
}
