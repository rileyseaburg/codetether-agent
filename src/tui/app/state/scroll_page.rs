//! Viewport-sized chat navigation.

impl super::AppState {
    pub(super) fn manual_chat_scroll(&self) -> usize {
        self.chat_viewport.manual_chat_scroll()
    }

    pub(crate) fn page_up(&mut self) {
        self.scroll_up(self.history_page.page_step());
    }

    pub(crate) fn page_down(&mut self) {
        self.scroll_down(self.history_page.page_step());
    }
}

#[cfg(test)]
#[path = "scroll_page_tests.rs"]
mod tests;
