//! Downward navigation and re-entry into follow mode.
//! The shared state owns viewport positions without application events.

impl crate::ChatScrollState {
    /// scroll down.
    ///
    /// # Arguments
    /// * `amount` — Lines to move.
    ///
    /// # Returns
    /// Updates this viewport in place.
    ///
    /// # Examples
    /// ```
    /// let mut s = codetether_chat_layout::ChatScrollState::default();
    /// s.scroll_down(10);
    /// ```
    pub fn scroll_down(&mut self, amount: usize) {
        if self.chat_scroll >= crate::SCROLL_BOTTOM {
            return;
        }
        let next = self.chat_scroll.saturating_add(amount);
        if next >= self.chat_last_max_scroll {
            self.scroll_to_bottom();
        } else {
            self.chat_scroll = next;
        }
    }
}
