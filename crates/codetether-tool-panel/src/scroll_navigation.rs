//! Tool preview navigation.
//! The shared state owns viewport positions without application events.

use crate::TOOL_PREVIEW_FOLLOW;
impl crate::ToolPreviewScrollState {
    /// scroll tool preview up.
    ///
    /// # Arguments
    /// * `amount` — Lines to move.
    ///
    /// # Returns
    /// Updates this viewport in place.
    ///
    /// # Examples
    /// ```
    /// let mut s = codetether_tool_panel::ToolPreviewScrollState::default();
    /// s.scroll_tool_preview_up(10);
    /// ```
    pub fn scroll_tool_preview_up(&mut self, amount: usize) {
        let base = if self.tool_preview_scroll >= TOOL_PREVIEW_FOLLOW {
            self.tool_preview_last_max_scroll
        } else {
            self.tool_preview_scroll
        };
        self.tool_preview_scroll = base.saturating_sub(amount);
    }
    /// scroll tool preview down.
    ///
    /// # Arguments
    /// * `amount` — Lines to move.
    ///
    /// # Returns
    /// Updates this viewport in place.
    ///
    /// # Examples
    /// ```
    /// let mut s = codetether_tool_panel::ToolPreviewScrollState::default();
    /// s.scroll_tool_preview_down(10);
    /// ```
    pub fn scroll_tool_preview_down(&mut self, amount: usize) {
        if self.tool_preview_scroll >= TOOL_PREVIEW_FOLLOW {
            return;
        }
        let next = self.tool_preview_scroll.saturating_add(amount);
        self.tool_preview_scroll = if next >= self.tool_preview_last_max_scroll {
            TOOL_PREVIEW_FOLLOW
        } else {
            next
        };
    }
}
