//! Tool-preview scrolling and auto-follow behavior.

impl super::super::AppState {
    pub fn scroll_tool_preview_up(&mut self, amount: usize) {
        self.tool_viewport.scroll_tool_preview_up(amount);
    }

    pub fn scroll_tool_preview_down(&mut self, amount: usize) {
        self.tool_viewport.scroll_tool_preview_down(amount);
    }

    pub fn reset_tool_preview_scroll(&mut self) {
        self.tool_viewport.reset_tool_preview_scroll();
    }

    pub fn set_tool_preview_max_scroll(&mut self, max_scroll: usize) {
        self.tool_viewport.set_tool_preview_max_scroll(max_scroll);
    }
}
