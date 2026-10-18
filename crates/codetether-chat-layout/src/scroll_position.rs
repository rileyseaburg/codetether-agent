//! Chat viewport positioning and content bounds.
//! The shared state owns viewport positions without application events.

impl crate::ChatScrollState {
    /// scroll to bottom.
    ///
    /// # Returns
    /// Updates this viewport in place.
    ///
    /// # Examples
    /// ```
    /// let mut s = codetether_chat_layout::ChatScrollState::default();
    /// s.scroll_to_bottom();
    /// ```
    pub fn scroll_to_bottom(&mut self) {
        self.chat_scroll = crate::SCROLL_BOTTOM;
        self.chat_auto_follow = true;
    }
    /// scroll to top.
    ///
    /// # Returns
    /// Updates this viewport in place.
    ///
    /// # Examples
    /// ```
    /// let mut s = codetether_chat_layout::ChatScrollState::default();
    /// s.scroll_to_top();
    /// ```
    pub fn scroll_to_top(&mut self) {
        self.chat_scroll = 0;
    }
    /// set chat max scroll.
    ///
    /// # Arguments
    /// * `max_scroll` — Measured maximum position.
    ///
    /// # Returns
    /// Updates this viewport in place.
    ///
    /// # Examples
    /// ```
    /// let mut s = codetether_chat_layout::ChatScrollState::default();
    /// s.set_chat_max_scroll(10);
    /// ```
    pub fn set_chat_max_scroll(&mut self, max_scroll: usize) {
        self.chat_last_max_scroll = max_scroll;
        if max_scroll == 0 {
            self.chat_scroll = 0;
        } else if self.chat_scroll < crate::SCROLL_BOTTOM {
            self.chat_scroll = self.chat_scroll.min(max_scroll);
        }
    }
}
