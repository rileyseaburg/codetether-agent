//! Ralph view detail scroll.
use crate::{RalphStoryInfo, RalphViewState};
impl RalphViewState {
    /// Scroll detail view down
    pub fn detail_scroll_down(&mut self, amount: usize) {
        self.detail_scroll = self.detail_scroll.saturating_add(amount);
    }

    /// Scroll detail view up
    pub fn detail_scroll_up(&mut self, amount: usize) {
        self.detail_scroll = self.detail_scroll.saturating_sub(amount);
    }

    /// Get the currently selected story
    pub fn selected_story(&self) -> Option<&RalphStoryInfo> {
        self.stories.get(self.selected_index)
    }
}
