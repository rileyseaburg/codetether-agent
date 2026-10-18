//! Bind shared saved scroll state to native viewport fields.
impl super::AppState {
    /// Save chat scroll state before leaving Chat view.
    pub fn save_scroll_state(&mut self) {
        self.saved_scroll.save(
            self.chat_viewport.chat_scroll,
            self.chat_viewport.chat_auto_follow,
            self.tool_viewport.tool_preview_scroll,
        );
    }
    /// Restore chat scroll state when returning to Chat view.
    pub fn restore_scroll_state(&mut self) {
        self.saved_scroll.restore(
            &mut self.chat_viewport.chat_scroll,
            &mut self.chat_viewport.chat_auto_follow,
            &mut self.tool_viewport.tool_preview_scroll,
        );
    }
}
