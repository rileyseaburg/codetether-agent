//! Native tool, message and approval presentation fields.
use super::Native;
use codetether_approval_core::interaction::LiveApprovalRequest;
use codetether_chat_model::ChatMessage;
impl codetether_chat_events::tools::View for Native<'_> {
    fn streaming_text(&mut self) -> &mut String {
        &mut self.app.state.streaming.streaming_text
    }
    fn messages(&mut self) -> &mut Vec<ChatMessage> {
        &mut self.app.state.messages
    }
    fn reset_tool_preview_scroll(&mut self) {
        self.app.state.reset_tool_preview_scroll();
    }
    fn start_pending_tool(&mut self, name: String) {
        self.app.state.start_pending_tool(name);
    }
    fn note_tool_completed(&mut self, name: String, duration_ms: u64, success: bool) {
        self.app
            .state
            .note_tool_completed(name, duration_ms, success);
    }
    fn set_status(&mut self, status: String) {
        self.app.state.status = status;
    }
    fn scroll_to_bottom(&mut self) {
        self.app.state.scroll_to_bottom();
    }
    fn approval_request(&mut self, request: LiveApprovalRequest) {
        super::approval::request(self.app, request);
    }
    fn approval_guidance(&self) -> String {
        crate::tui::ui::trust_status::approval_guidance()
    }
}
