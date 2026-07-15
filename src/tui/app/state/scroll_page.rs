//! Viewport-sized chat navigation.

impl super::AppState {
    pub(super) fn manual_chat_scroll(&self) -> usize {
        if self.chat_scroll >= crate::tui::constants::SCROLL_BOTTOM {
            self.chat_last_max_scroll
        } else {
            self.chat_scroll
        }
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
