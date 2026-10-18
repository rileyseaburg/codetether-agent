//! Chat and tool-preview scroll methods.
//!
//! Sentinel scheme: `chat_scroll >= 1_000_000` ("follow latest") is clamped
//! to `chat_last_max_scroll` at render time. `tool_preview_scroll` mirrors
//! the same scheme via [`TOOL_PREVIEW_FOLLOW`] so the tool panel auto-follows
//! the most recent activity until the user manually scrolls up.

/// Sentinel: tool panel auto-follows (scrolls to bottom of latest activity).
pub use codetether_tool_panel::TOOL_PREVIEW_FOLLOW;

#[path = "scroll_tool_preview.rs"]
mod tool_preview;

impl super::AppState {
    pub fn scroll_up(&mut self, amount: usize) {
        if let Some(deficit) = self.chat_viewport.scroll_up(amount) {
            self.history_page
                .request_older(deficit, self.messages.len());
        }
    }

    pub fn scroll_down(&mut self, amount: usize) {
        self.chat_viewport.scroll_down(amount);
    }

    /// Set sentinel value — clamped to actual content height at render time.
    /// Re-engages [`codetether_chat_layout::ChatScrollState::chat_auto_follow`] so subsequent session events
    /// keep the user pinned to the latest output.
    pub fn scroll_to_bottom(&mut self) {
        self.chat_viewport.scroll_to_bottom();
    }

    pub fn scroll_to_top(&mut self) {
        self.chat_viewport.scroll_to_top();
    }

    pub fn set_chat_max_scroll(&mut self, max_scroll: usize) {
        self.chat_viewport.set_chat_max_scroll(max_scroll);
    }
}
