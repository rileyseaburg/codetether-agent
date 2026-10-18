//! Application stream state for shared chat events.
/// Supporting implementation for [crate::declare_chat_state].
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_chat_stream {
    ($native:ident, $limit:path) => {
        impl $crate::StreamView for $native<'_> {
            fn auto_follow(&self) -> bool {
                self.0.state.chat_viewport.chat_auto_follow
            }
            fn scroll_to_bottom(&mut self) {
                self.0.state.chat_viewport.scroll_to_bottom();
            }
            fn note_text_token(&mut self) {
                self.0.state.request_timing.note_text_token();
            }
            fn replace_streaming_text(&mut self, text: String) {
                self.0
                    .state
                    .message_cache
                    .replace_streaming_text(&mut self.0.state.streaming, text);
            }
            fn record_streaming_chars(&mut self, count: usize) {
                self.0.state.streaming.record_streaming_chars(count);
            }
            fn streaming_len(&self) -> usize {
                self.0.state.streaming.streaming_text.len()
            }
            fn max_streaming_bytes(&self) -> usize {
                $limit
            }
            fn clear_streaming_text(&mut self) {
                self.0
                    .state
                    .message_cache
                    .clear_streaming_text(&mut self.0.state.streaming);
            }
            fn reset_tool_preview_scroll(&mut self) {
                self.0.state.tool_viewport.reset_tool_preview_scroll();
            }
            fn messages(&mut self) -> &mut Vec<$crate::__private::ChatMessage> {
                &mut self.0.state.messages
            }
            fn set_status(&mut self, status: String) {
                self.0.state.status = status;
            }
        }
    };
}
