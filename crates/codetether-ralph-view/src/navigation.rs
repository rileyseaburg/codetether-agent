//! Ralph view navigation.
use crate::RalphViewState;
impl RalphViewState {
    /// Move selection up
    pub fn select_prev(&mut self) {
        if self.stories.is_empty() {
            return;
        }
        self.selected_index = self.selected_index.saturating_sub(1);
        self.list_state.select(Some(self.selected_index));
    }

    /// Move selection down
    pub fn select_next(&mut self) {
        if self.stories.is_empty() {
            return;
        }
        self.selected_index = (self.selected_index + 1).min(self.stories.len() - 1);
        self.list_state.select(Some(self.selected_index));
    }

    /// Enter detail mode for the selected story
    pub fn enter_detail(&mut self) {
        if !self.stories.is_empty() {
            self.detail_mode = true;
            self.detail_scroll = 0;
        }
    }

    /// Exit detail mode
    pub fn exit_detail(&mut self) {
        self.detail_mode = false;
        self.detail_scroll = 0;
    }
}
