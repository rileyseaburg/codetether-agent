//! Frontend view operations for tool events.
/// Supporting implementation for [crate::declare_tool_events].
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_tool_event_view {
    ($native:ident, $approval:path, $guidance:path) => {
        impl $crate::tools::View for $native<'_> {
            fn streaming_text(&mut self) -> &mut String {
                &mut self.app.state.streaming.streaming_text
            }
            fn messages(&mut self) -> &mut Vec<$crate::__private::ChatMessage> {
                &mut self.app.state.messages
            }
            fn reset_tool_preview_scroll(&mut self) {
                self.app.state.tool_viewport.reset_tool_preview_scroll();
            }
            fn start_pending_tool(&mut self, name: String) {
                self.app.state.tool_timing.start_pending_tool(name);
            }
            fn note_tool_completed(&mut self, name: String, duration_ms: u64, success: bool) {
                self.app
                    .state
                    .tool_timing
                    .note_tool_completed(name, duration_ms, success);
            }
            fn set_status(&mut self, status: String) {
                self.app.state.status = status;
            }
            fn scroll_to_bottom(&mut self) {
                self.app.state.chat_viewport.scroll_to_bottom();
            }
            fn approval_request(&mut self, request: $crate::__private::LiveApprovalRequest) {
                $approval(self.app, request);
            }
            fn approval_guidance(&self) -> String {
                $guidance()
            }
        }
    };
}
