//! Manual upward navigation and older-history demand.
//! The shared state owns viewport positions without application events.

impl crate::ChatScrollState {
    /// Scroll upward and disengage automatic following.
    ///
    /// # Arguments
    /// * `amount` — Requested number of lines.
    ///
    /// # Returns
    /// `Some(deficit)` at the top, otherwise `None`; hosts may request older history.
    ///
    /// # Examples
    /// ```
    /// let mut s = codetether_chat_layout::ChatScrollState::default();
    /// assert_eq!(s.scroll_up(3), Some(3));
    /// ```
    pub fn scroll_up(&mut self, amount: usize) -> Option<usize> {
        // Manual scroll-up disengages auto-follow so streaming output
        // stops yanking the user back to the bottom of the chat.
        self.chat_auto_follow = false;
        let base = self.manual_chat_scroll();
        self.chat_scroll = base.saturating_sub(amount);
        if self.chat_scroll == 0 {
            Some(amount.saturating_sub(base))
        } else {
            None
        }
    }
    /// Resolve the current position for manual navigation.
    ///
    /// # Returns
    /// The last measured bound when following, otherwise the current position.
    ///
    /// # Examples
    /// ```
    /// let s = codetether_chat_layout::ChatScrollState::default();
    /// assert_eq!(s.manual_chat_scroll(), 0);
    /// ```
    pub fn manual_chat_scroll(&self) -> usize {
        if self.chat_scroll >= crate::SCROLL_BOTTOM {
            self.chat_last_max_scroll
        } else {
            self.chat_scroll
        }
    }
}
