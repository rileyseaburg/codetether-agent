//! Save/restore scroll state when switching view modes.
//!
//! Streaming events fire `scroll_to_bottom` while the user is away from
//! Chat (e.g. inspecting `/bus`), which would otherwise yank their
//! scrollback to the latest message on return. Capture the scroll
//! position on the way out and replay it on the way back.

impl super::AppState {
    /// Save chat scroll state before leaving Chat view.
    pub fn save_scroll_state(&mut self) {
        self.saved_chat_scroll = self.chat_scroll;
        self.saved_chat_auto_follow = self.chat_auto_follow;
        self.saved_tool_preview_scroll = self.tool_preview_scroll;
    }

    /// Restore chat scroll state when returning to Chat view.
    pub fn restore_scroll_state(&mut self) {
        self.chat_scroll = self.saved_chat_scroll;
        self.chat_auto_follow = self.saved_chat_auto_follow;
        self.tool_preview_scroll = self.saved_tool_preview_scroll;
    }
}
