//! Application UI projection for backward-history page application.
use super::{HistoryPageState, source::NativeSource};
use crate::tui::app::state::AppState;
use codetether_chat_model::ChatMessage;
impl codetether_history_page::interactive::View<NativeSource> for AppState {
    fn pager(&mut self) -> &mut HistoryPageState {
        &mut self.history_page
    }
    fn follow_latest(&self) -> bool {
        self.chat_viewport.chat_scroll >= crate::tui::constants::SCROLL_BOTTOM
    }
    fn manual_scroll(&self) -> usize {
        self.manual_chat_scroll()
    }
    fn cached_lines(&self) -> usize {
        self.message_cache.cached_message_lines.len()
    }
    fn last_max_scroll(&self) -> usize {
        self.chat_viewport.chat_last_max_scroll
    }
    fn messages(&mut self) -> &mut Vec<ChatMessage> {
        &mut self.messages
    }
    fn clear_cache(&mut self) {
        crate::tui::app::message_cache_invalidate::clear(self);
    }
    fn set_status(&mut self, status: String) {
        self.status = status;
    }
    fn set_scroll(&mut self, scroll: usize) {
        self.chat_viewport.chat_scroll = scroll;
    }
    fn set_auto_follow(&mut self, follow: bool) {
        self.chat_viewport.chat_auto_follow = follow;
    }
}
