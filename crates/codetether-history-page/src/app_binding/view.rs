//! Existing AppState viewport and message fields for history-page application.
/// Internal history view declaration.
#[doc(hidden)]
#[macro_export]
macro_rules! __app_history_view {
    ($state:ty, $source:ty, $clear:path, $bottom:path) => {
        impl $crate::interactive::View<$source> for $state {
            fn pager(&mut self) -> &mut $crate::interactive::HistoryPageState<$source> {
                &mut self.history_page
            }
            fn follow_latest(&self) -> bool {
                self.chat_viewport.chat_scroll >= $bottom
            }
            fn manual_scroll(&self) -> usize {
                self.chat_viewport.manual_chat_scroll()
            }
            fn cached_lines(&self) -> usize {
                self.message_cache.cached_message_lines.len()
            }
            fn last_max_scroll(&self) -> usize {
                self.chat_viewport.chat_last_max_scroll
            }
            fn messages(&mut self) -> &mut ::std::vec::Vec<$crate::app_binding::ChatMessage> {
                &mut self.messages
            }
            fn clear_cache(&mut self) {
                $clear(self);
            }
            fn set_status(&mut self, status: ::std::string::String) {
                self.status = status;
            }
            fn set_scroll(&mut self, scroll: usize) {
                self.chat_viewport.chat_scroll = scroll;
            }
            fn set_auto_follow(&mut self, follow: bool) {
                self.chat_viewport.chat_auto_follow = follow;
            }
        }
    };
}
