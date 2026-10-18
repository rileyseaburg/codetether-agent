//! In-memory paging and viewport fixture for migrated regression tests.
use super::{HistoryPageState, View};
use codetether_chat_model::ChatMessage;
use codetether_provider_core::Message;
pub(super) struct Source;
#[async_trait::async_trait]
impl crate::HistorySource for Source {
    async fn load_tail(_: &str, _: usize) -> Result<Vec<Message>, String> {
        Ok(Vec::new())
    }
    async fn read_messages(_: &str, _: usize, _: usize) -> Result<Vec<Message>, String> {
        Ok(Vec::new())
    }
}
pub(super) struct Viewport {
    pub chat_scroll: usize,
    pub chat_auto_follow: bool,
}
impl Default for Viewport {
    fn default() -> Self {
        Self {
            chat_scroll: 0,
            chat_auto_follow: true,
        }
    }
}
#[derive(Default)]
pub(super) struct ViewState {
    pub history_page: HistoryPageState<Source>,
    pub chat_viewport: Viewport,
    messages: Vec<ChatMessage>,
    status: String,
}
impl View<Source> for ViewState {
    fn pager(&mut self) -> &mut HistoryPageState<Source> {
        &mut self.history_page
    }
    fn follow_latest(&self) -> bool {
        false
    }
    fn manual_scroll(&self) -> usize {
        self.chat_viewport.chat_scroll
    }
    fn cached_lines(&self) -> usize {
        0
    }
    fn last_max_scroll(&self) -> usize {
        0
    }
    fn messages(&mut self) -> &mut Vec<ChatMessage> {
        &mut self.messages
    }
    fn clear_cache(&mut self) {}
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
