//! Native presentation adapter for shared transcript synchronization.
mod metrics;
use crate::tui::app::state::App;
use codetether_chat_model::ChatMessage;
use codetether_chat_transcript::sync::{Metrics, View};
use codetether_provider_core::Message;
impl View for App {
    fn replace_messages(&mut self, messages: Vec<ChatMessage>) {
        self.state.messages = messages;
    }
    fn reset_history(
        &mut self,
        source_id: String,
        boundary: &[Message],
        depth: usize,
        has_older: bool,
    ) {
        self.state
            .history_page
            .reset(source_id, boundary, depth, has_older);
    }
    fn set_history_before(&mut self, before: usize) {
        self.state.history_page.set_before(before);
    }
    fn clear_cache(&mut self) {
        crate::tui::app::message_cache_invalidate::clear(&mut self.state);
    }
    fn metrics(&mut self) -> Metrics<'_> {
        metrics::fields(&mut self.state)
    }
    fn reset_tool_preview_scroll(&mut self) {
        self.state.tool_viewport.reset_tool_preview_scroll();
    }
    fn set_tool_preview_max_scroll(&mut self, max: usize) {
        self.state.tool_viewport.set_tool_preview_max_scroll(max);
    }
    fn scroll_to_bottom(&mut self) {
        self.state.scroll_to_bottom();
    }
}
