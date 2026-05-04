//! Chat and tool-preview scroll methods.
//!
//! Sentinel scheme: `chat_scroll >= 1_000_000` ("follow latest") is clamped
//! to `chat_last_max_scroll` at render time. `tool_preview_scroll` mirrors
//! the same scheme via [`TOOL_PREVIEW_FOLLOW`] so the tool panel auto-follows
//! the most recent activity until the user manually scrolls up.

/// Sentinel: tool panel auto-follows (scrolls to bottom of latest activity).
pub const TOOL_PREVIEW_FOLLOW: usize = 1_000_000;

impl super::AppState {
    pub fn scroll_up(&mut self, amount: usize) {
        let base = if self.chat_scroll >= 1_000_000 { self.chat_last_max_scroll } else { self.chat_scroll };
        self.chat_scroll = base.saturating_sub(amount);
    }

    pub fn scroll_down(&mut self, amount: usize) {
        if self.chat_scroll >= 1_000_000 { return; }
        let next = self.chat_scroll.saturating_add(amount);
        if next >= self.chat_last_max_scroll { self.scroll_to_bottom(); } else { self.chat_scroll = next; }
    }

    /// Set sentinel value — clamped to actual content height at render time.
    pub fn scroll_to_bottom(&mut self) { self.chat_scroll = 1_000_000; }

    pub fn scroll_to_top(&mut self) { self.chat_scroll = 0; }

    pub fn set_chat_max_scroll(&mut self, max_scroll: usize) {
        self.chat_last_max_scroll = max_scroll;
        if max_scroll == 0 {
            self.chat_scroll = 0;
        } else if self.chat_scroll < 1_000_000 {
            self.chat_scroll = self.chat_scroll.min(max_scroll);
        }
    }

    pub fn scroll_tool_preview_up(&mut self, amount: usize) {
        // Manual scroll cancels auto-follow.
        let base = if self.tool_preview_scroll >= TOOL_PREVIEW_FOLLOW {
            self.tool_preview_last_max_scroll
        } else {
            self.tool_preview_scroll
        };
        self.tool_preview_scroll = base.saturating_sub(amount);
    }

    pub fn scroll_tool_preview_down(&mut self, amount: usize) {
        if self.tool_preview_scroll >= TOOL_PREVIEW_FOLLOW { return; }
        let next = self.tool_preview_scroll.saturating_add(amount);
        // Re-enable auto-follow when hitting bottom.
        self.tool_preview_scroll = if next >= self.tool_preview_last_max_scroll { TOOL_PREVIEW_FOLLOW } else { next };
    }

    pub fn reset_tool_preview_scroll(&mut self) { self.tool_preview_scroll = TOOL_PREVIEW_FOLLOW; }

    pub fn set_tool_preview_max_scroll(&mut self, max_scroll: usize) {
        self.tool_preview_last_max_scroll = max_scroll;
        self.tool_preview_scroll = if self.tool_preview_scroll >= TOOL_PREVIEW_FOLLOW {
            max_scroll
        } else {
            self.tool_preview_scroll.min(max_scroll)
        };
    }
}
