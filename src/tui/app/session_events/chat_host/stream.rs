//! Native stream timing, preview and message fields.
use super::Native;
use codetether_chat_model::ChatMessage;
impl codetether_chat_events::StreamView for Native<'_> {
    fn auto_follow(&self) -> bool {
        self.0.state.chat_viewport.chat_auto_follow
    }
    fn scroll_to_bottom(&mut self) {
        self.0.state.scroll_to_bottom();
    }
    fn note_text_token(&mut self) {
        self.0.state.note_text_token();
    }
    fn replace_streaming_text(&mut self, text: String) {
        self.0.state.replace_streaming_text(text);
    }
    fn record_streaming_chars(&mut self, count: usize) {
        self.0.state.record_streaming_chars(count);
    }
    fn streaming_len(&self) -> usize {
        self.0.state.streaming.streaming_text.len()
    }
    fn max_streaming_bytes(&self) -> usize {
        crate::tui::constants::MAX_STREAMING_TEXT_BYTES
    }
    fn clear_streaming_text(&mut self) {
        self.0.state.clear_streaming_text();
    }
    fn reset_tool_preview_scroll(&mut self) {
        self.0.state.reset_tool_preview_scroll();
    }
    fn messages(&mut self) -> &mut Vec<ChatMessage> {
        &mut self.0.state.messages
    }
    fn set_status(&mut self, status: String) {
        self.0.state.status = status;
    }
}
