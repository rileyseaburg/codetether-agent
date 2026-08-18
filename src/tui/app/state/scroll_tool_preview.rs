//! Tool-preview scrolling and auto-follow behavior.

use super::TOOL_PREVIEW_FOLLOW;

impl super::super::AppState {
    pub fn scroll_tool_preview_up(&mut self, amount: usize) {
        let base = if self.tool_preview_scroll >= TOOL_PREVIEW_FOLLOW {
            self.tool_preview_last_max_scroll
        } else {
            self.tool_preview_scroll
        };
        self.tool_preview_scroll = base.saturating_sub(amount);
    }

    pub fn scroll_tool_preview_down(&mut self, amount: usize) {
        if self.tool_preview_scroll >= TOOL_PREVIEW_FOLLOW {
            return;
        }
        let next = self.tool_preview_scroll.saturating_add(amount);
        self.tool_preview_scroll = if next >= self.tool_preview_last_max_scroll {
            TOOL_PREVIEW_FOLLOW
        } else {
            next
        };
    }

    pub fn reset_tool_preview_scroll(&mut self) {
        self.tool_preview_scroll = TOOL_PREVIEW_FOLLOW;
    }

    pub fn set_tool_preview_max_scroll(&mut self, max_scroll: usize) {
        self.tool_preview_last_max_scroll = max_scroll;
        if self.tool_preview_scroll < TOOL_PREVIEW_FOLLOW {
            self.tool_preview_scroll = self.tool_preview_scroll.min(max_scroll);
        }
    }
}
