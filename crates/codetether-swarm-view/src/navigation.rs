//! Swarm view navigation.
use crate::SwarmViewState;
impl SwarmViewState {
    /// Move selection up
    pub fn select_prev(&mut self) {
        if self.subtasks.is_empty() {
            return;
        }
        self.selected_index = (self.selected_index + self.subtasks.len() - 1) % self.subtasks.len();
        self.list_state.select(Some(self.selected_index));
    }

    /// Move selection down
    pub fn select_next(&mut self) {
        if self.subtasks.is_empty() {
            return;
        }
        self.selected_index = (self.selected_index + 1) % self.subtasks.len();
        self.list_state.select(Some(self.selected_index));
    }

    /// Enter detail mode for the selected subtask
    pub fn enter_detail(&mut self) {
        if !self.subtasks.is_empty() {
            self.detail_mode = true;
            self.detail_scroll = 0;
        }
    }

    /// Exit detail mode, return to list
    pub fn exit_detail(&mut self) {
        self.detail_mode = false;
        self.detail_scroll = 0;
    }
}
