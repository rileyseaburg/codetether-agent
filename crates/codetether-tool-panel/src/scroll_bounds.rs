//! Tool preview bounds.
//! The shared state owns viewport positions without application events.

use crate::TOOL_PREVIEW_FOLLOW;
impl crate::ToolPreviewScrollState {
    /// reset tool preview scroll.
    ///
    /// # Returns
    /// Updates this viewport in place.
    ///
    /// # Examples
    /// ```
    /// let mut s = codetether_tool_panel::ToolPreviewScrollState::default();
    /// s.reset_tool_preview_scroll();
    /// ```
    pub fn reset_tool_preview_scroll(&mut self) {
        self.tool_preview_scroll = TOOL_PREVIEW_FOLLOW;
    }
    /// set tool preview max scroll.
    ///
    /// # Arguments
    /// * `max_scroll` — Measured maximum position.
    ///
    /// # Returns
    /// Updates this viewport in place.
    ///
    /// # Examples
    /// ```
    /// let mut s = codetether_tool_panel::ToolPreviewScrollState::default();
    /// s.set_tool_preview_max_scroll(10);
    /// ```
    pub fn set_tool_preview_max_scroll(&mut self, max_scroll: usize) {
        self.tool_preview_last_max_scroll = max_scroll;
        if self.tool_preview_scroll < TOOL_PREVIEW_FOLLOW {
            self.tool_preview_scroll = self.tool_preview_scroll.min(max_scroll);
        }
    }
}
