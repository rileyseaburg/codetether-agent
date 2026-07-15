//! Viewport measurements shared by page navigation and render anchoring.

use super::HistoryPageState;

impl HistoryPageState {
    pub(crate) fn set_viewport_height(&mut self, height: usize) {
        self.viewport_height = height.max(1);
    }

    pub(crate) fn page_step(&self) -> usize {
        self.viewport_height.saturating_sub(1).max(1)
    }

    pub(super) fn known_total_lines(&self, max_scroll: usize) -> usize {
        max_scroll.saturating_add(self.viewport_height)
    }
}
