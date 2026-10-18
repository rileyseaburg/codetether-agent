//! Swarm view detail scroll.
use crate::SubTaskInfo;
use crate::SwarmViewState;
impl SwarmViewState {
    /// Scroll detail view down
    pub fn detail_scroll_down(&mut self, amount: usize) {
        self.detail_scroll = self.detail_scroll.saturating_add(amount);
    }

    /// Scroll detail view up
    pub fn detail_scroll_up(&mut self, amount: usize) {
        self.detail_scroll = self.detail_scroll.saturating_sub(amount);
    }

    /// Get the currently selected subtask
    pub fn selected_subtask(&self) -> Option<&SubTaskInfo> {
        self.subtasks.get(self.selected_index)
    }
}
