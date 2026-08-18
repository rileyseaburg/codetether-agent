//! Chat and tool-preview scroll methods.
//!
//! Sentinel scheme: `chat_scroll >= 1_000_000` ("follow latest") is clamped
//! to `chat_last_max_scroll` at render time. `tool_preview_scroll` mirrors
//! the same scheme via [`TOOL_PREVIEW_FOLLOW`] so the tool panel auto-follows
//! the most recent activity until the user manually scrolls up.

/// Sentinel: tool panel auto-follows (scrolls to bottom of latest activity).
pub const TOOL_PREVIEW_FOLLOW: usize = 1_000_000;

#[path = "scroll_tool_preview.rs"]
mod tool_preview;

impl super::AppState {
    pub fn scroll_up(&mut self, amount: usize) {
        // Manual scroll-up disengages auto-follow so streaming output
        // stops yanking the user back to the bottom of the chat.
        self.chat_auto_follow = false;
        let base = self.manual_chat_scroll();
        self.chat_scroll = base.saturating_sub(amount);
        if self.chat_scroll == 0 {
            self.history_page
                .request_older(amount.saturating_sub(base), self.messages.len());
        }
    }

    pub fn scroll_down(&mut self, amount: usize) {
        if self.chat_scroll >= 1_000_000 {
            return;
        }
        let next = self.chat_scroll.saturating_add(amount);
        if next >= self.chat_last_max_scroll {
            self.scroll_to_bottom();
        } else {
            self.chat_scroll = next;
        }
    }

    /// Set sentinel value — clamped to actual content height at render time.
    /// Re-engages [`AppState::chat_auto_follow`] so subsequent session events
    /// keep the user pinned to the latest output.
    pub fn scroll_to_bottom(&mut self) {
        self.chat_scroll = 1_000_000;
        self.chat_auto_follow = true;
    }

    pub fn scroll_to_top(&mut self) {
        self.chat_scroll = 0;
    }

    pub fn set_chat_max_scroll(&mut self, max_scroll: usize) {
        self.chat_last_max_scroll = max_scroll;
        if max_scroll == 0 {
            self.chat_scroll = 0;
        } else if self.chat_scroll < 1_000_000 {
            self.chat_scroll = self.chat_scroll.min(max_scroll);
        }
    }
}
