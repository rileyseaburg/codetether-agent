//! Chat and tool-preview scroll methods.
//!
//! Implements the sentinel-value scheme: `chat_scroll >= 1_000_000` means
//! "follow latest" and is clamped to `chat_last_max_scroll` at render time.

impl super::AppState {
    pub fn scroll_up(&mut self, amount: usize) {
        if self.chat_scroll >= 1_000_000 {
            self.chat_scroll = self.chat_last_max_scroll.saturating_sub(amount);
        } else {
            self.chat_scroll = self.chat_scroll.saturating_sub(amount);
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
    pub fn scroll_to_bottom(&mut self) {
        self.chat_scroll = 1_000_000;
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

    pub fn scroll_tool_preview_up(&mut self, amount: usize) {
        self.tool_preview_scroll = self.tool_preview_scroll.saturating_sub(amount);
    }

    pub fn scroll_tool_preview_down(&mut self, amount: usize) {
        let next = self.tool_preview_scroll.saturating_add(amount);
        self.tool_preview_scroll = next.min(self.tool_preview_last_max_scroll);
    }

    pub fn reset_tool_preview_scroll(&mut self) {
        self.tool_preview_scroll = 0;
    }

    pub fn set_tool_preview_max_scroll(&mut self, max_scroll: usize) {
        self.tool_preview_last_max_scroll = max_scroll;
        self.tool_preview_scroll = self.tool_preview_scroll.min(max_scroll);
    }
}
